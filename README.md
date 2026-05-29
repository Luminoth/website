# EnergonSoftware Website

Full-stack personal website: Angular 21 frontend + Axum (Rust) backend + AWS DynamoDB.

## Prerequisites

- **Docker** — `sudo apt install docker-buildx`
- **Node.js** — use [fnm](https://github.com/Schniz/fnm) or [nvm](https://github.com/nvm-sh/nvm). See [Angular version compatibility](https://angular.dev/reference/versions) for the required version.
- **Angular CLI** — `npm install -g @angular/cli`
- **Rust** — via [rustup](https://rustup.rs/)
- **Task** — task runner used for all build/deploy commands. See [taskfile.dev](https://taskfile.dev) for install instructions.

## Quick start

```sh
task ui:init     # install frontend dependencies
task api:run     # start the Rust backend on :8000
task ui:run      # start the Angular dev server on :4200
```

Or use Docker Compose to run the API in a container:

```sh
task start       # docker compose up -d
task stop        # docker compose stop + rm
```

## Available tasks

Run `task` with no arguments to list all tasks.

**Docker Compose**

| Task | Description |
|---|---|
| `task start` | Start API via docker compose (detached) |
| `task stop` | Stop and remove docker compose containers |

**API (`api:*`)**

| Task | Description |
|---|---|
| `task api:run` | Start the API locally via cargo |
| `task api:run-docker` | Build and run the API in Docker |
| `task api:build` | Build the API Docker image |
| `task api:check` | Fast compile check (`cargo check`) |
| `task api:lint` | Run Rust linter (`cargo clippy`) |
| `task api:fmt` | Format Rust source (`cargo fmt`) |
| `task api:fmt-check` | Check Rust formatting without modifying files |
| `task api:test` | Run Rust unit tests |
| `task api:update` | Update Rust dependencies (`cargo update`) |
| `task api:deploy` | Build, push to ECR, force ECS redeploy |

**UI (`ui:*`)**

| Task | Description |
|---|---|
| `task ui:init` | Install frontend npm dependencies |
| `task ui:run` | Start the Angular dev server on :4200 |
| `task ui:build` | Build the Angular app for production |
| `task ui:lint` | Run Angular linter (`ng lint`) |
| `task ui:test` | Run Angular tests headless, single run |
| `task ui:test-watch` | Run Angular tests in watch mode |
| `task ui:update` | Update frontend npm dependencies (`npm update`) |
| `task ui:deploy` | Build and sync frontend to S3 + invalidate CloudFront |

**Static assets (`static:*`)**

| Task | Description |
|---|---|
| `task static:deploy` | Sync static assets to S3 |
| `task static:deploy-dryrun` | Dry-run sync of static assets (no changes made) |

**Aggregate**

| Task | Description |
|---|---|
| `task lint` | Run all linters (`api:lint` + `ui:lint`) |
| `task test` | Run all tests (`api:test` + `ui:test`) |
| `task update` | Update all dependencies (`api:update` + `ui:update`) |
| `task clean` | Remove build artifacts |
| `task really-clean` | Clean and remove Docker images |

## Updating dependencies

### Backend

```sh
task api:update
```

### Frontend

See `energonsoftware/package.json` for current versions.

For minor/patch updates:
```sh
task ui:update
```

For major Angular version upgrades, follow the [Angular update guide](https://angular.dev/update-guide):
```sh
ng update @angular/cli @angular/core@<major>
ng update @angular/material @angular/cdk@<major>
```

## Environment

Copy `.env.example` to `.env` and fill in:

```
AWS_ACCOUNT=   # AWS account ID (for ECR/ECS deploy)
UI_DISTRIBUTION=  # CloudFront distribution ID (for cache invalidation)
```

## Notes

- Warp removed in `292496f850aebd54875d40f2a041afcb60777fac`
- The API requires `tokio_unstable` (via `.cargo/config.toml`) for `console-subscriber` support
