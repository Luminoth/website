use std::net::SocketAddr;
use std::ops::Deref;
use std::time::Instant;

use axum::extract::ConnectInfo;
use http::header;
use tracing::info;

use crate::util::{self, OptFmt};

// TODO: if we can get an id into the Span then we can use
// on_request and on_response instead of tracing_wrapper

#[allow(dead_code)]
pub fn on_request<B>(request: &http::Request<B>, span: &tracing::Span) {
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

    info!(
        target: "energonsoftware::api",
        "req:{} {}{} \"{} {} {:?}\" \"{}\" \"{}\"",
        OptFmt(span.id().map(|x| x.into_u64())),
        OptFmt(remote_addr),
        if forwarded { " (forwarded)" } else { "" },
        request.method(),
        request.uri(),
        request.version(),
        OptFmt(util::get_request_header(&request, header::REFERER).map(str::to_owned)),
        OptFmt(util::get_request_header(&request, header::USER_AGENT).map(str::to_owned)),
    );
}

#[allow(dead_code)]
pub fn on_response<B>(
    response: &http::Response<B>,
    latency: std::time::Duration,
    span: &tracing::Span,
) {
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

    Ok(response)
}
