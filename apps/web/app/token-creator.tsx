"use client";

import { FormEvent, useState } from "react";

type CreatedToken = {
  id: string;
  name: string;
  token: string;
};

export function TokenCreator() {
  const [name, setName] = useState("ACC Collector");
  const [createdToken, setCreatedToken] = useState<CreatedToken | null>(null);
  const [status, setStatus] = useState<"idle" | "creating" | "created" | "copied" | "error">(
    "idle"
  );
  const [error, setError] = useState<string | null>(null);

  async function createToken(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setStatus("creating");
    setError(null);

    try {
      const response = await fetch("/api/collectors/token", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ name })
      });

      if (!response.ok) {
        throw new Error("Could not create collector token.");
      }

      const token = (await response.json()) as CreatedToken;
      setCreatedToken(token);
      setStatus("created");
    } catch (caught) {
      setStatus("error");
      setError(caught instanceof Error ? caught.message : "Could not create collector token.");
    }
  }

  async function copyToken() {
    if (!createdToken) {
      return;
    }

    await navigator.clipboard.writeText(createdToken.token);
    setStatus("copied");
  }

  return (
    <section className="panel token-panel" aria-labelledby="token-title">
      <div>
        <h2 id="token-title">Collector token</h2>
        <p>Create a token for the Windows collector and paste it into the collector app.</p>
      </div>

      <form className="token-form" onSubmit={createToken}>
        <label htmlFor="collector-name">Collector name</label>
        <div className="token-row">
          <input
            id="collector-name"
            name="name"
            value={name}
            onChange={(event) => setName(event.target.value)}
            placeholder="ACC Collector"
          />
          <button type="submit" disabled={status === "creating"}>
            {status === "creating" ? "Creating..." : "Create token"}
          </button>
        </div>
      </form>

      {error ? <p className="message error">{error}</p> : null}

      {createdToken ? (
        <div className="token-result">
          <div>
            <span className="eyebrow">Created for</span>
            <strong>{createdToken.name}</strong>
          </div>
          <code>{createdToken.token}</code>
          <button type="button" className="secondary" onClick={copyToken}>
            {status === "copied" ? "Copied" : "Copy token"}
          </button>
        </div>
      ) : null}
    </section>
  );
}
