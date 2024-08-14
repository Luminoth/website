use std::time::Instant;

use http::header;
use tracing::info;

use crate::util::{self, OptFmt};

// using this instead of dealing with TraceLayer
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
    let remote_addr = util::get_forwarded_addr(&request);
    if remote_addr.is_none() {
        //remote_addr = request.remote_addr();
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
