use crate::metrics::Metrics;
use crate::utils::print_request_status;
use crate::utils::is_cloudflare_error;
use std::sync::Arc;
use std::sync::atomic::Ordering;

const ALLOWED_CONTENT_TYPES: [&str; 4] = [
    "text/html",
    "application/xhtml+xml",
    "application/xhtml",
    "text/plain",
];

pub async fn try_tunnel_request(
    url: &str,
    metrics: &Arc<Metrics>,
) -> Result<String, Box<dyn std::error::Error>> {
    metrics.total.fetch_add(1, Ordering::Relaxed);
    metrics.tunnel.fetch_add(1, Ordering::Relaxed);

    let original_url = url.to_string();

    let parsed_url = if !url.contains("://") {
        format!("http://{}", url)
    } else {
        url.to_string()
    };

    let url_parts: Vec<&str> = parsed_url.splitn(2, "://").collect();
    if url_parts.len() != 2 {
        return Err("Invalid URL format".into());
    }

    let scheme = url_parts[0];
    let rest = url_parts[1];
    let tunnel_url = format!("{}{}:/{}", *crate::PROXY_TUNNEL_URL, scheme, rest);

    match crate::proxy::TUNNEL_CLIENT.get(&tunnel_url).send().await {
        Ok(response) => {
            let status = response.status();
            
            // check content type before downloading body
            if let Some(content_type) = response.headers().get("content-type") {
                let content_type = content_type.to_str().unwrap_or_default().to_lowercase();
                if !ALLOWED_CONTENT_TYPES.iter().any(|&allowed| content_type.contains(allowed)) {
                    print_request_status(&original_url, "TUNNEL", "SKIPPED", Some(&content_type));
                    return Err(format!("Unsupported content type: {}", content_type).into());
                }
            }

            let text = response.text().await?;
            if status == 403 || text.contains("403 Forbidden") {
                print_request_status(&original_url, "TUNNEL", "FAILED", Some("403 Forbidden"));
                return Err("403 Forbidden".into());
            }
            if is_cloudflare_error(&text) {
                print_request_status(
                    &original_url,
                    "TUNNEL",
                    "FAILED",
                    Some("Cloudflare error detected"),
                );
                Err("Cloudflare error in response content".into())
            } else {
                print_request_status(&original_url, "TUNNEL", "SUCCESS", None);
                Ok(text)
            }
        }
        Err(e) => {
            print_request_status(&original_url, "TUNNEL", "FAILED", Some(&e.to_string()));
            Err(e.into())
        }
    }
}
