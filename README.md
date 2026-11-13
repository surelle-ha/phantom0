# Phantom0

**A lightweight, easy-to-deploy Security Information and Event Management (SIEM) platform.**

Phantom0 is an open-source SIEM project designed to make security monitoring, event collection, and log analysis simpler and more accessible.

Built with performance, simplicity, and deployability in mind, Phantom0 combines lightweight Rust-based agents, modular Rust microservices, PostgreSQL, and a modern web interface powered by Nuxt and Vue.js.

The goal is simple: provide a practical SIEM solution that is easy to install, configure, manage, and scale without the operational complexity often associated with traditional SIEM platforms.

---

## Overview

Phantom0 follows a modular architecture consisting of three primary components:

- **Agent / Daemon** – A lightweight Rust-based service deployed on monitored endpoints. Responsible for collecting system information, security events, and telemetry, then forwarding relevant data to the Phantom0 backend.
- **Backend Microservices** – Independent Rust services responsible for processing, managing, and exposing collected security data through APIs.
- **Web Dashboard** – A modern frontend built with Nuxt and Vue.js, providing a centralized interface for monitoring, managing, and analyzing security events.

## Technology Stack

| Component | Technology |
|---|---|
| Agent / Daemon | Rust |
| Backend Services | Rust |
| Backend Framework | Axum |
| Database | PostgreSQL |
| ORM | SeaORM |
| Client Framework | Dioxus |
| Deployment | Docker / Docker Compose |
| Reverse Proxy | Traefik |

## Architecture

```text
                  PHANTOM0
              Security Platform
                      |
          +-----------+-----------+
          |                       |
     Monitored Endpoints      Web Dashboard
          |                       |
     +----+----+              Nuxt / Vue.js
     |         |                   |
  Agent 1   Agent 2                 |
     |         |                   |
     +----+----+                   |
          |                        |
          v                        v
      Backend Microservices <--- API
          |
     +----+----------------+
     |          |           |
   Events     Agents      Users
     |          |           |
     +----------+-----------+
                |
            PostgreSQL
```

## Core Principles

- **Simplicity First** – Easy installation, configuration, and deployment.
- **Lightweight Agents** – Efficient endpoint monitoring with minimal resource overhead.
- **Modular Architecture** – Independent services with clearly defined responsibilities.
- **Performance** – Leverage Rust's performance, memory safety, and concurrency.
- **Self-Hosted** – Designed to run on infrastructure you control.
- **Extensibility** – Built to accommodate additional monitoring capabilities and integrations.

## Project Structure

The repository is organized as a modular system, separating endpoint monitoring, backend services, and frontend applications.

```text
phantom0/
├── agent/              # Rust endpoint agent / daemon
├── services/           # Rust backend microservices
│   ├── api-gateway/    # API entry point (if required)
│   ├── users/          # User management
│   ├── events/         # Security event management
│   └── ...
├── frontend/           # Nuxt / Vue.js dashboard
├── infrastructure/    # Deployment and infrastructure configs
├── docker-compose.yml  # Container orchestration
└── README.md
```

*Note: The directory structure is illustrative and may evolve as the project develops.*

## Deployment

Phantom0 is designed with containerized deployment in mind.

The intended deployment approach uses Docker Compose to orchestrate the backend services, PostgreSQL, frontend, and reverse proxy.

```bash
git clone https://github.com/UnknownSecurity/phantom0.git
cd phantom0
docker compose up -d
```

The services are available through Traefik on port 80:

| Service | Endpoint | Health check |
|---|---|---|
| User management | `/api/users/` | `/api/users/health` |
| Identity and access | `/api/identity/` | `/api/identity/health` |
| Groups | `/api/groups/` | `/api/groups/health` |
| Notifications | `/api/notifications/` | `/api/notifications/health` |

The Dioxus client is served at `/`.

For local client development, run:

```bash
dx serve --platform web --package client
```

Deployment instructions and configuration requirements will be documented as the project matures.

## Development Status

<sub>Early development</sub>

Phantom0 is currently under active development. Its architecture and core components are being established.

Planned capabilities include:

- [ ] Rust-based endpoint agent / daemon
- [ ] Endpoint registration and management
- [ ] System telemetry collection
- [ ] Security event collection
- [ ] Centralized event management
- [ ] Event search and filtering
- [ ] Web-based monitoring dashboard
- [ ] Authentication and access control
- [ ] Docker Compose deployment
- [ ] Alerting and notification system
- [ ] Detection rules and threat monitoring

## Contributing

Contributions, suggestions, and ideas are welcome.

If you're interested in improving Phantom0, feel free to open an issue or submit a pull request.

## License

License information will be added.

---

**Phantom0**  
*Security monitoring, without the unnecessary complexity.*
