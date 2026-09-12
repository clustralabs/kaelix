"use client";

import Link from "next/link";
import { useCallback, useEffect, useState } from "react";

type ServiceState = "checking" | "online" | "offline";

type DockerVersion = {
  version?: string;
  api_version?: string;
  os?: string;
  arch?: string;
};

function ServiceBadge({ state }: { state: ServiceState }) {
  const labels = { checking: "Checking", online: "Operational", offline: "Unavailable" };
  return (
    <span className={`badge badge-${state}`}>
      <span className="badge-dot" aria-hidden="true" />
      {labels[state]}
    </span>
  );
}

export default function Home() {
  const [database, setDatabase] = useState<ServiceState>("checking");
  const [docker, setDocker] = useState<ServiceState>("checking");
  const [dockerVersion, setDockerVersion] = useState<DockerVersion | null>(null);
  const [lastChecked, setLastChecked] = useState<string | null>(null);

  const checkServices = useCallback(async () => {
    setDatabase("checking");
    setDocker("checking");
    const [databaseResponse, dockerResponse] = await Promise.allSettled([
      fetch("/api/up", { cache: "no-store" }),
      fetch("/api/version", { cache: "no-store" }),
    ]);

    setDatabase(
      databaseResponse.status === "fulfilled" && databaseResponse.value.ok ? "online" : "offline",
    );
    if (dockerResponse.status === "fulfilled" && dockerResponse.value.ok) {
      setDocker("online");
      setDockerVersion(await dockerResponse.value.json());
    } else {
      setDocker("offline");
      setDockerVersion(null);
    }
    setLastChecked(new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }));
  }, []);

  useEffect(() => {
    const initialCheck = window.setTimeout(() => void checkServices(), 0);
    return () => window.clearTimeout(initialCheck);
  }, [checkServices]);

  const allOnline = database === "online" && docker === "online";

  return (
    <main className="shell">
      <nav className="nav" aria-label="Main navigation">
        <Link className="brand" href="/" aria-label="kaelix home">
          <span className="brand-mark">K</span>
          <span>kaelix</span>
        </Link>
        <span className="nav-context">control plane / local</span>
      </nav>

      <section className="hero" aria-labelledby="page-title">
        <div>
          <p className="eyebrow">System overview</p>
          <h1 id="page-title">Your infrastructure,<br /><em>at a glance.</em></h1>
          <p className="hero-copy">A quiet control surface for the services that keep your applications running.</p>
        </div>
        <div className={`overall-status ${allOnline ? "is-online" : ""}`} role="status">
          <span className="overall-icon" aria-hidden="true">{allOnline ? "✓" : "·"}</span>
          <div>
            <strong>{allOnline ? "All systems operational" : "Checking services"}</strong>
            <span>{lastChecked ? `Last checked at ${lastChecked}` : "Connecting to the API"}</span>
          </div>
        </div>
      </section>

      <section className="service-section" aria-labelledby="services-title">
        <div className="section-heading">
          <div>
            <p className="eyebrow">01 / Dependencies</p>
            <h2 id="services-title">Connected services</h2>
          </div>
          <button className="refresh-button" onClick={() => void checkServices()} type="button">
            <span aria-hidden="true">↻</span> Refresh
          </button>
        </div>

        <div className="service-grid">
          <article className="service-card">
            <div className="service-card-top"><span className="service-icon db-icon" aria-hidden="true">▦</span><ServiceBadge state={database} /></div>
            <h3>Postgres</h3>
            <p>Primary data store</p>
            <div className="service-meta"><span>postgres:5432</span><span className="meta-live">● {database === "online" ? "reachable" : "awaiting connection"}</span></div>
          </article>
          <article className="service-card">
            <div className="service-card-top"><span className="service-icon docker-icon" aria-hidden="true">◇</span><ServiceBadge state={docker} /></div>
            <h3>Docker engine</h3>
            <p>Container runtime</p>
            <div className="service-meta"><span>{dockerVersion?.version ? `v${dockerVersion.version}` : "docker.sock"}</span><span className="meta-live">● {dockerVersion?.os ?? "local"}</span></div>
          </article>
        </div>
      </section>

      <section className="empty-section" aria-labelledby="deployments-title">
        <div><p className="eyebrow">02 / Workloads</p><h2 id="deployments-title">Deployments</h2></div>
        <div className="empty-state"><span className="empty-number">00</span><div><h3>No deployments yet</h3><p>Your managed containers will appear here when the API is ready.</p></div><button className="primary-button" type="button" disabled>New deployment <span>→</span></button></div>
      </section>

      <footer><span>kaelix / v0.1</span><span>Built for clear skies</span></footer>
    </main>
  );
}
