CREATE TABLE "user_preferences" (
	"user_id" text PRIMARY KEY NOT NULL,
	"preferred_language" text DEFAULT 'en' NOT NULL,
	"safe_search" boolean DEFAULT true NOT NULL,
	"autocomplete" boolean DEFAULT true NOT NULL,
	"instant_results" boolean DEFAULT true NOT NULL,
	"ai_summarise" boolean DEFAULT true NOT NULL,
	"anonymous_queries" boolean DEFAULT true NOT NULL,
	"analytics_enabled" boolean DEFAULT true NOT NULL,
	"ai_personalization" boolean DEFAULT true NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
ALTER TABLE "user_preferences" ADD CONSTRAINT "user_preferences_user_id_user_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."user"("id") ON DELETE cascade ON UPDATE no action;