use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::LazyLock;
use std::time::Instant;

use axum::extract::{ConnectInfo, MatchedPath};
use http::header;
use opentelemetry::{KeyValue, global, metrics::Histogram};

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

/// Resolves the requester's address for the request span, plus how it was
/// resolved: `"x-forwarded-for"` when the header supplied it (production,
/// behind the load balancer), `"socket"` when falling back to the raw TCP
/// peer via `ConnectInfo` (local/direct connections).
fn remote_addr<B>(request: &http::Request<B>) -> (Option<SocketAddr>, &'static str) {
    if let Some(addr) = util::get_forwarded_addr(request) {
        return (Some(addr), "x-forwarded-for");
    }

    let addr = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|x| x.deref())
        .copied();
    (addr, "socket")
}

/// `MakeSpan` for `TraceLayer`: skips span creation entirely for AWS health
/// checks (via `Span::none()`) so they don't show up as logs or traces
/// either, and carries the request's identifying fields (method, URI,
/// remote address, referer, user agent) on the span itself so `TraceLayer`'s
/// own request/response log lines get them for free, instead of each log
/// call site formatting its own copy.
pub fn make_span(request: &axum::extract::Request) -> tracing::Span {
    if is_health_check(request) {
        return tracing::Span::none();
    }

    let (remote_addr, remote_addr_source) = remote_addr(request);

    tracing::info_span!(
        "request",
        method = %request.method(),
        uri = %request.uri(),
        version = ?request.version(),
        remote_addr = %OptFmt(remote_addr),
        remote_addr_source,
        referer = %OptFmt(util::get_request_header(request, header::REFERER)),
        user_agent = %OptFmt(util::get_request_header(request, header::USER_AGENT)),
    )
}

/// Records the `http.server.request.duration` OTel metric. Kept separate
/// from `make_span`/`TraceLayer` because metrics and logs are independent
/// pipelines with independent exporters (see `otel.rs`) - this only touches
/// the former. Skips AWS health checks, same as `make_span`, so they don't
/// pollute the metric either.
pub async fn record_request_duration(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    if is_health_check(&request) {
        return next.run(request).await;
    }

    let method = request.method().clone();
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(|matched_path| matched_path.as_str().to_owned())
        .unwrap_or_else(|| request.uri().path().to_owned());

    let start = Instant::now();
    let response = next.run(request).await;

    HTTP_SERVER_REQUEST_DURATION.record(
        start.elapsed().as_secs_f64(),
        &[
            KeyValue::new("http.request.method", method.as_str().to_owned()),
            KeyValue::new("http.route", route),
            KeyValue::new(
                "http.response.status_code",
                response.status().as_u16() as i64,
            ),
        ],
    );

    response
}
