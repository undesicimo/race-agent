import { z } from "zod";

export const SimSchema = z.enum(["acc", "iracing", "assetto_corsa", "generic"]);

export const TyreSampleSchema = z.object({
  pressurePsi: z.number().nullable().optional(),
  temperatureC: z.number().nullable().optional(),
  wear: z.number().min(0).max(1).nullable().optional()
});

export const TelemetrySampleSchema = z.object({
  timestamp: z.string().datetime(),
  speedKph: z.number().nonnegative().nullable().optional(),
  rpm: z.number().int().nonnegative().nullable().optional(),
  gear: z.number().int().nullable().optional(),
  throttle: z.number().min(0).max(1).nullable().optional(),
  brake: z.number().min(0).max(1).nullable().optional(),
  clutch: z.number().min(0).max(1).nullable().optional(),
  steering: z.number().min(-1).max(1).nullable().optional(),
  lapTimeMs: z.number().int().nonnegative().nullable().optional(),
  normalizedTrackPosition: z.number().min(0).max(1).nullable().optional(),
  fuelLiters: z.number().nonnegative().nullable().optional(),
  tyres: z
    .object({
      frontLeft: TyreSampleSchema.optional(),
      frontRight: TyreSampleSchema.optional(),
      rearLeft: TyreSampleSchema.optional(),
      rearRight: TyreSampleSchema.optional()
    })
    .optional()
});

export const TelemetryBatchSchema = z.object({
  sim: SimSchema,
  sessionId: z.string().uuid().optional(),
  collectorId: z.string().uuid().optional(),
  samples: z.array(TelemetrySampleSchema).min(1).max(1000)
});

export const SessionStartSchema = z.object({
  sim: SimSchema,
  collectorId: z.string().uuid().optional(),
  carName: z.string().min(1).optional(),
  trackName: z.string().min(1).optional(),
  startedAt: z.string().datetime().optional()
});

export const SessionEndSchema = z.object({
  sessionId: z.string().uuid(),
  endedAt: z.string().datetime().optional()
});

export const HeartbeatSchema = z.object({
  sim: SimSchema,
  collectorId: z.string().uuid().optional(),
  status: z.enum(["starting", "connected", "uploading", "idle", "error"]),
  message: z.string().optional(),
  timestamp: z.string().datetime().optional()
});

export type TelemetryBatch = z.infer<typeof TelemetryBatchSchema>;
export type TelemetrySample = z.infer<typeof TelemetrySampleSchema>;
export type SessionStart = z.infer<typeof SessionStartSchema>;
export type SessionEnd = z.infer<typeof SessionEndSchema>;
export type Heartbeat = z.infer<typeof HeartbeatSchema>;
