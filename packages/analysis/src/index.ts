import type { TelemetrySample } from "@sim-telemetry/telemetry-schema";

export interface LapSummary {
  sampleCount: number;
  maxSpeedKph: number | null;
  averageThrottle: number | null;
  averageBrake: number | null;
}

export function summarizeLap(samples: TelemetrySample[]): LapSummary {
  const maxSpeedKph = maxDefined(samples.map((sample) => sample.speedKph));

  return {
    sampleCount: samples.length,
    maxSpeedKph,
    averageThrottle: averageDefined(samples.map((sample) => sample.throttle)),
    averageBrake: averageDefined(samples.map((sample) => sample.brake))
  };
}

export function compareLaps(lapA: TelemetrySample[], lapB: TelemetrySample[]) {
  return {
    lapA: summarizeLap(lapA),
    lapB: summarizeLap(lapB)
  };
}

function averageDefined(values: Array<number | null | undefined>) {
  const defined = values.filter((value): value is number => typeof value === "number");
  if (defined.length === 0) {
    return null;
  }

  return defined.reduce((sum, value) => sum + value, 0) / defined.length;
}

function maxDefined(values: Array<number | null | undefined>) {
  const defined = values.filter((value): value is number => typeof value === "number");
  if (defined.length === 0) {
    return null;
  }

  return Math.max(...defined);
}
