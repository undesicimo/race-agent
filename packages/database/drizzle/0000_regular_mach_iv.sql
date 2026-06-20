CREATE TYPE "public"."sim" AS ENUM('acc', 'iracing', 'assetto_corsa', 'generic');--> statement-breakpoint
CREATE TABLE "apikey" (
	"id" text PRIMARY KEY NOT NULL,
	"config_id" text DEFAULT 'default' NOT NULL,
	"name" text,
	"start" text,
	"prefix" text,
	"key" text NOT NULL,
	"reference_id" text NOT NULL,
	"refill_interval" integer,
	"refill_amount" integer,
	"last_refill_at" timestamp with time zone,
	"enabled" boolean,
	"rate_limit_enabled" boolean,
	"rate_limit_time_window" integer,
	"rate_limit_max" integer,
	"request_count" integer,
	"remaining" integer,
	"last_request" timestamp with time zone,
	"expires_at" timestamp with time zone,
	"created_at" timestamp with time zone NOT NULL,
	"updated_at" timestamp with time zone NOT NULL,
	"permissions" text,
	"metadata" text
);
--> statement-breakpoint
CREATE TABLE "cars" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"sim" "sim" NOT NULL,
	"name" text NOT NULL
);
--> statement-breakpoint
CREATE TABLE "collector_heartbeats" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"collector_id" uuid,
	"sim" "sim" NOT NULL,
	"status" text NOT NULL,
	"message" text,
	"recorded_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "collectors" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"name" text,
	"token_hash" text,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "laps" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"session_id" uuid NOT NULL,
	"lap_number" integer NOT NULL,
	"lap_time_ms" integer,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "session_events" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"session_id" uuid NOT NULL,
	"type" text NOT NULL,
	"payload" jsonb,
	"recorded_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE "sessions" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"collector_id" uuid,
	"sim" "sim" NOT NULL,
	"car_id" uuid,
	"track_id" uuid,
	"started_at" timestamp with time zone DEFAULT now() NOT NULL,
	"ended_at" timestamp with time zone
);
--> statement-breakpoint
CREATE TABLE "telemetry_samples" (
	"id" uuid DEFAULT gen_random_uuid() NOT NULL,
	"session_id" uuid NOT NULL,
	"lap_id" uuid,
	"recorded_at" timestamp with time zone NOT NULL,
	"speed_kph" double precision,
	"rpm" integer,
	"gear" integer,
	"throttle" double precision,
	"brake" double precision,
	"clutch" double precision,
	"steering" double precision,
	"lap_time_ms" integer,
	"normalized_track_position" double precision,
	"fuel_liters" double precision,
	"tyres" jsonb,
	CONSTRAINT "telemetry_samples_id_recorded_at_pk" PRIMARY KEY("id","recorded_at")
);
--> statement-breakpoint
CREATE TABLE "tracks" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"sim" "sim" NOT NULL,
	"name" text NOT NULL
);
--> statement-breakpoint
ALTER TABLE "collector_heartbeats" ADD CONSTRAINT "collector_heartbeats_collector_id_collectors_id_fk" FOREIGN KEY ("collector_id") REFERENCES "public"."collectors"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "laps" ADD CONSTRAINT "laps_session_id_sessions_id_fk" FOREIGN KEY ("session_id") REFERENCES "public"."sessions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "session_events" ADD CONSTRAINT "session_events_session_id_sessions_id_fk" FOREIGN KEY ("session_id") REFERENCES "public"."sessions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "sessions" ADD CONSTRAINT "sessions_collector_id_collectors_id_fk" FOREIGN KEY ("collector_id") REFERENCES "public"."collectors"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "sessions" ADD CONSTRAINT "sessions_car_id_cars_id_fk" FOREIGN KEY ("car_id") REFERENCES "public"."cars"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "sessions" ADD CONSTRAINT "sessions_track_id_tracks_id_fk" FOREIGN KEY ("track_id") REFERENCES "public"."tracks"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "telemetry_samples" ADD CONSTRAINT "telemetry_samples_session_id_sessions_id_fk" FOREIGN KEY ("session_id") REFERENCES "public"."sessions"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "telemetry_samples" ADD CONSTRAINT "telemetry_samples_lap_id_laps_id_fk" FOREIGN KEY ("lap_id") REFERENCES "public"."laps"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "apikey_config_id_idx" ON "apikey" USING btree ("config_id");--> statement-breakpoint
CREATE INDEX "apikey_reference_id_idx" ON "apikey" USING btree ("reference_id");--> statement-breakpoint
CREATE UNIQUE INDEX "cars_sim_name_idx" ON "cars" USING btree ("sim","name");--> statement-breakpoint
CREATE INDEX "collector_heartbeats_collector_recorded_at_idx" ON "collector_heartbeats" USING btree ("collector_id","recorded_at");--> statement-breakpoint
CREATE INDEX "telemetry_samples_session_recorded_at_idx" ON "telemetry_samples" USING btree ("session_id","recorded_at");--> statement-breakpoint
CREATE INDEX "telemetry_samples_recorded_at_idx" ON "telemetry_samples" USING btree ("recorded_at");--> statement-breakpoint
CREATE UNIQUE INDEX "tracks_sim_name_idx" ON "tracks" USING btree ("sim","name");
