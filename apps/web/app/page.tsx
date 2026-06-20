import { TokenCreator } from "./token-creator";

export default function HomePage() {
  return (
    <div className="stack">
      <section>
        <h1>Sim telemetry dashboard</h1>
        <p>
          Ingest ACC telemetry, store normalized sessions, and analyze driving
          performance.
        </p>
      </section>
      <section className="panel">
        <h2>Milestone 1</h2>
        <p>
          First target: ACC collector uploads speed, RPM, gear, throttle, brake,
          and lap time into Postgres for a live session page.
        </p>
      </section>
      <TokenCreator />
    </div>
  );
}
