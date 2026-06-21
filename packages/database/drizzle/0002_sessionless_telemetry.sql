ALTER TABLE "telemetry_samples" DROP CONSTRAINT IF EXISTS "telemetry_samples_session_id_sessions_id_fk";--> statement-breakpoint
ALTER TABLE "telemetry_samples" ALTER COLUMN "session_id" DROP NOT NULL;--> statement-breakpoint
