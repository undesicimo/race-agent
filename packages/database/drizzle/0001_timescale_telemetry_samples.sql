-- Custom SQL migration file, put your code below! --
CREATE EXTENSION IF NOT EXISTS timescaledb;--> statement-breakpoint
SELECT create_hypertable('telemetry_samples', 'recorded_at', if_not_exists => TRUE);
