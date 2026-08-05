use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Instant;

use axum::extract::{ConnectInfo, MatchedPath};
use http::header;
use opentelemetry::{KeyValue, global, metrics::Histogram};
use tower_http::trace::{DefaultMakeSpan, MakeSpan};
use tracing::{Level, info};

use crate::otel::{SERVICE_NAME, http_server_request_duration_boundaries};
use crate::util::{self, OptFmt};

// Per OTel HTTP semantic conventions: http.server.request.duration, unit seconds.
// This is what New Relic's OTel APM experience uses to build the Summary view
// (throughput/response time/error rate) - without it only the legacy span view
// is available, regardless of how much trace data is exported.
static HTTP_SERVER_REQUEST_DURATION: LazyLock<Histogram<f64>> = LazyLock::new(|| {
    global::meter(SERVICE_NAME)
        .f64_histogram("http.server.request.duration")
        .with_unit("s")
        .with_description("Duration of HTTP server requests")
        .with_boundaries(http_server_request_duration_boundaries())
        .build()
});

fn is_health_check<B>(request: &http::Request<B>) -> bool {
    request.uri().path() == "/healthz"
}

/// `MakeSpan` for `TraceLayer`: skips span creation entirely for AWS health
/// checks (via `Span::none()`) so they don't show up as traces either, on top
/// of already being excluded from logs and the request duration metric.
pub fn make_span(request: &axum::extract::Request) -> tracing::Span {
    if is_health_check(request) {
        return tracing::Span::none();
    }

    DefaultMakeSpan::new().level(Level::INFO).make_span(request)
}

// TODO: if we can get an id into the Span then we can use
// on_request and on_response instead of tracing_wrapper

#[allow(dead_code)]
pub fn on_request<B>(request: &http::Request<B>, span: &tracing::Span) {
    let mut forwarded = true;
    let mut remote_addr = util::get_forwarded_addr(request);
    if remote_addr.is_none() {
        remote_addr = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|x| x.deref())
            .copied();
        forwarded = false;
    }

    let user_agent = util::get_request_header(request, header::USER_AGENT);

    // don't log AWS health check requests
    if !is_health_check(request) {
        info!(
            target: "energonsoftware::api",
            "req:{} {}{} \"{} {} {:?}\" \"{}\" \"{}\"",
            OptFmt(span.id().map(|x| x.into_u64())),
            OptFmt(remote_addr),
            if forwarded { " (forwarded)" } else { "" },
            request.method(),
            request.uri(),
            request.version(),
            OptFmt(util::get_request_header(request, header::REFERER)),
            OptFmt(user_agent),
        );
    }
}

#[allow(dead_code)]
pub fn on_response<B>(
    response: &http::Response<B>,
    latency: std::time::Duration,
    span: &tracing::Span,
) {
    // TODO: need to not log if the User-Agent is the health checker

    info!(
        target: "energonsoftware::api",
        "req:{} {} {:?}",
        OptFmt(span.id().map(|x| x.into_u64())),
        response.status().as_u16(),
        latency,
    );
}

// using this instead of TraceLayer
// because I want to log everything about the
// request / response together
pub async fn tracing_wrapper(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<impl axum::response::IntoResponse, axum::response::Response> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(|matched_path| matched_path.as_str().to_owned());
    let is_health_check = is_health_check(&request);
    let referer = util::get_request_header(&request, header::REFERER).map(str::to_owned);
    let user_agent = util::get_request_header(&request, header::USER_AGENT).map(str::to_owned);

    let mut forwarded = true;
    let mut remote_addr = util::get_forwarded_addr(&request);
    if remote_addr.is_none() {
        remote_addr = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|x| x.deref())
            .copied();
        forwarded = false;
    }

    let now = Instant::now();
    let response = next.run(request).await;
    let elapsed = now.elapsed();

    // don't log or record metrics for AWS health check requests
    if !is_health_check {
        HTTP_SERVER_REQUEST_DURATION.record(
            elapsed.as_secs_f64(),
            &[
                KeyValue::new("http.request.method", method.as_str().to_owned()),
                KeyValue::new("http.route", route.unwrap_or_else(|| uri.path().to_owned())),
                KeyValue::new(
                    "http.response.status_code",
                    response.status().as_u16() as i64,
                ),
            ],
        );

        info!(
            target: "energonsoftware::api",
            "{}{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
            OptFmt(remote_addr),
            if forwarded { " (forwarded)" } else { "" },
            method,
            uri,
            version,
            response.status().as_u16(),
            OptFmt(referer),
            OptFmt(user_agent),
            elapsed,
        );
    }

    Ok(response)
}
