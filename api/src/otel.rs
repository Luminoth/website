use std::collections::HashMap;

use opentelemetry::global;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter, WithHttpConfig};
use opentelemetry_sdk::{
    Resource,
    logs::SdkLoggerProvider,
    metrics::{PeriodicReader, SdkMeterProvider},
    trace::SdkTracerProvider,
};

pub(crate) const SERVICE_NAME: &str = "energonsoftware-api";

/// Recommended bucket boundaries (in seconds) for the `http.server.request.duration`
/// metric, per the OTel HTTP semantic conventions.
const HTTP_SERVER_REQUEST_DURATION_BOUNDARIES: [f64; 14] = [
    0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
];

/// Bucket boundaries to use for the `http.server.request.duration` histogram.
pub fn http_server_request_duration_boundaries() -> Vec<f64> {
    HTTP_SERVER_REQUEST_DURATION_BOUNDARIES.to_vec()
}

pub struct OtelProviders {
    pub tracer_provider: SdkTracerProvider,
    pub meter_provider: SdkMeterProvider,
    pub logger_provider: SdkLoggerProvider,
}

impl OtelProviders {
    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.tracer_provider.shutdown()?;
        self.meter_provider.shutdown()?;
        self.logger_provider.shutdown()?;

        Ok(())
    }
}

fn new_relic_headers() -> HashMap<String, String> {
    match std::env::var("NEW_RELIC_LICENSE_KEY") {
        // New Relic's OTLP ingest expects the license key as an `api-key` header
        // rather than the standard `OTEL_EXPORTER_OTLP_HEADERS` env var.
        Ok(license_key) => HashMap::from([("api-key".to_string(), license_key)]),
        Err(_) => HashMap::new(),
    }
}

/// Builds and registers OTLP exporters pointed at
/// `OTEL_EXPORTER_OTLP_ENDPOINT`, or does nothing if that variable isn't set
/// (e.g. local dev).
///
/// New Relic's APM experience is driven by metrics matching its HTTP semantic
/// conventions (e.g. `http.server.request.duration`), not by spans, so both
/// pipelines need to be registered for the OTel APM view to populate. The log
/// pipeline isn't registered as a global provider — its caller wraps the
/// returned `SdkLoggerProvider` in a tracing-subscriber layer directly.
///
/// Runs before the tracing subscriber is installed (its result builds the OTel
/// layer that goes into that subscriber), so it can't log anything itself —
/// the caller logs the outcome once the subscriber is live.
pub fn init_otel_providers() -> anyhow::Result<Option<OtelProviders>> {
    if std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_err() {
        return Ok(None);
    }

    let headers = new_relic_headers();

    let span_exporter = SpanExporter::builder()
        .with_http()
        .with_headers(headers.clone())
        .build()?;

    let metric_exporter = MetricExporter::builder()
        .with_http()
        .with_headers(headers.clone())
        .build()?;

    let log_exporter = LogExporter::builder()
        .with_http()
        .with_headers(headers)
        .build()?;

    let service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| SERVICE_NAME.to_string());
    let resource = Resource::builder().with_service_name(service_name).build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(resource.clone())
        .build();

    let reader = PeriodicReader::builder(metric_exporter).build();
    let meter_provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(resource.clone())
        .build();

    let logger_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(resource)
        .build();

    global::set_tracer_provider(tracer_provider.clone());
    global::set_meter_provider(meter_provider.clone());

    Ok(Some(OtelProviders {
        tracer_provider,
        meter_provider,
        logger_provider,
    }))
}
