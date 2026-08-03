use std::collections::HashMap;

use opentelemetry::global;
use opentelemetry_otlp::{SpanExporter, WithHttpConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};

const SERVICE_NAME: &str = "energonsoftware-api";

/// Builds and registers an OTLP trace exporter pointed at `OTEL_EXPORTER_OTLP_ENDPOINT`,
/// or does nothing if that variable isn't set (e.g. local dev).
///
/// New Relic's OTLP ingest expects the license key as an `api-key` header rather than
/// the standard `OTEL_EXPORTER_OTLP_HEADERS` env var, so `NEW_RELIC_LICENSE_KEY` is
/// translated into that header here.
pub fn init_tracer_provider() -> anyhow::Result<Option<SdkTracerProvider>> {
    if std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_err() {
        return Ok(None);
    }

    let mut exporter_builder = SpanExporter::builder().with_http();

    if let Ok(license_key) = std::env::var("NEW_RELIC_LICENSE_KEY") {
        exporter_builder =
            exporter_builder.with_headers(HashMap::from([("api-key".to_string(), license_key)]));
    }

    let exporter = exporter_builder.build()?;

    let service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| SERVICE_NAME.to_string());

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder().with_service_name(service_name).build())
        .build();

    global::set_tracer_provider(provider.clone());

    Ok(Some(provider))
}
