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
task init        # install frontend dependencies
task run-api     # start the Rust backend on :8000
task run-ui      # start the Angular dev server on :4200
```

Or use Docker Compose to run the API in a container:

```sh
task start       # docker compose up -d
task stop        # docker compose stop + rm
```

## Available tasks

Run `task` with no arguments to list all tasks.

**Setup & development**

| Task | Description |
|---|---|
| `task init` | Install frontend npm dependencies |
| `task run-api` | Start the API locally via cargo |
| `task run-api-docker` | Build and run the API in Docker |
| `task run-ui` | Start the Angular dev server on :4200 |
| `task start` | Start API via docker compose (detached) |
| `task stop` | Stop and remove docker compose containers |

**Build**

| Task | Description |
|---|---|
| `task build-api` | Build the API Docker image |
| `task build-ui` | Build the Angular app for production |

**Check / lint / format**

| Task | Description |
|---|---|
| `task check` | Fast compile check (`cargo check`) |
| `task clippy` | Run Rust linter (`cargo clippy`) |
| `task fmt` | Format Rust source (`cargo fmt`) |
| `task fmt-check` | Check Rust formatting without modifying files |

**Test**

| Task | Description |
|---|---|
| `task test-api` | Run Rust unit tests |
| `task test-ui` | Run Angular tests headless, single run |
| `task test-ui-watch` | Run Angular tests in watch mode |

**Update**

| Task | Description |
|---|---|
| `task update-api` | Update Rust dependencies (`cargo update`) |
| `task update-ui` | Update frontend npm dependencies (`npm update`) |

**Clean**

| Task | Description |
|---|---|
| `task clean` | Remove build artifacts |
| `task really-clean` | Clean and remove Docker images |

**Deploy** (requires `.env`)

| Task | Description |
|---|---|
| `task deploy-api` | Build, push to ECR, force ECS redeploy |
| `task deploy-ui` | Build and sync frontend to S3 + invalidate CloudFront |
| `task deploy-static` | Sync static assets to S3 |
| `task deploy-static-dryrun` | Dry-run sync of static assets (no changes made) |

## Updating dependencies

### Backend

```sh
cargo update
```

### Frontend

See `energonsoftware/package.json` for current versions.

For minor/patch updates:
```sh
cd energonsoftware && npm update
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
