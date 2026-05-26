# EnergonSoftware Website — Claude Context

## Project structure

Rust/Angular monorepo:

```
api/          Axum web API (Rust binary)
core/         Shared Rust library (DynamoDB abstractions)
energonsoftware/  Angular 21 frontend SPA
static/       Static assets served from S3
share/        JSON data files (WoW addons, macros, screenshots)
Taskfile.yml  All build/run/deploy commands
```

## Build and run

```sh
task init           # npm install for frontend
task run-api        # cargo run API on :8000
task run-ui         # ng serve on :4200
task build-api      # docker buildx build
task build-ui       # ng build (production)
task check          # cargo check (fast)
task clippy         # cargo clippy
task fmt            # cargo fmt
task fmt-check      # cargo fmt --check
task test-api       # cargo test
task test-ui        # ng test --no-watch --browsers=ChromeHeadless
task test-ui-watch  # ng test (watch mode)
task update-api     # cargo update
task update-ui      # npm update
```

Run `task` with no args for the full task list.

## Tech stack

**Backend:**
- Rust 2024 edition, Axum 0.8, Tokio 1.52
- AWS SDK: DynamoDB via `aws-sdk-dynamodb` + custom `dynamodb_expression` crate
- Logging: `tracing` + `tracing-subscriber` (structured, respects `RUST_LOG`)
- CLI args via Clap 4; `.env` file via `dotenvy`

**Frontend:**
- Angular 21 (NgModule-based, not standalone components)
- Angular Material 21 + ng-bootstrap 20 + Bootstrap 5
- State: no store library; services return `lastValueFrom(HttpClient.get(...))`
- Styling: SCSS with Angular Material theming
- Tests: Karma + Jasmine

## Key conventions

- `#![deny(warnings)]` is set in `api/src/main.rs` — the build will fail on any Rust warning
- DynamoDB table/index constants live in `api/src/constants.rs` (`ITEMS_TABLE`, `TYPE_TIMESTAMP_INDEX`)
- API routes are versioned under `/v1/`
- Environment files: `environment.ts` (dev, points at localhost:8000), `environment.prod.ts` (prod, points at api.energonsoftware.org)
- CORS: non-prod mode allows both the configured origin and `localhost:4200`
- Logging level defaults to `info`; override with `RUST_LOG` env var

## Deployment

Requires `.env` with `AWS_ACCOUNT` and `UI_DISTRIBUTION`. Targets:
- API → AWS ECR + ECS (us-west-2)
- Frontend → S3 bucket `energonsoftware-website` + CloudFront
- Static assets → S3 bucket `energonsoftware-static`

## Known issues / TODO

See `TODO.md` for the current list. Key items:
- `tokio_unstable` is required by `console-subscriber` (see `.cargo/config.toml`)
- WoW data migration from `.ini` to `.json` is incomplete
- Download links are broken
- No CI/CD pipeline exists — all deploys are manual via `task`
- ESLint not yet configured (`ng lint` will report no lint target)
