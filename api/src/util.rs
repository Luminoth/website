use std::fmt;
use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use http::{Request, header::AsHeaderName};

/// Displays an `Option` as its inner value, or `"-"` when `None`.
// copied from warp's log filter
pub struct OptFmt<T>(pub Option<T>);

impl<T: fmt::Display> fmt::Display for OptFmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref t) = self.0 {
            fmt::Display::fmt(t, f)
        } else {
            f.write_str("-")
        }
    }
}

/// Returns the value of a request header as a `&str`, or `None` if the header
/// is absent or contains non-UTF-8 bytes.
pub fn get_request_header<B, K>(request: &Request<B>, header: K) -> Option<&str>
where
    K: AsHeaderName,
{
    let header = request.headers().get(header);
    if let Some(header) = header
        && let Ok(header) = header.to_str()
    {
        return Some(header);
    }
    None
}

fn get_forwarded_for<B>(request: &Request<B>) -> Option<&str> {
    // TODO: not sure if header::FORWARDED works here or not
    let forwarded_for = get_request_header(request, "X-Forwarded-For");
    if let Some(forwarded_for) = forwarded_for {
        let addrs = forwarded_for.split(',').collect::<Vec<&str>>();
        if !addrs.is_empty() {
            return Some(addrs[0]);
        }
    }
    None
}

/// Returns a `SocketAddr` combining the first IP from `X-Forwarded-For` with
/// the port from the underlying `ConnectInfo` (or port 0 if unavailable).
/// Returns `None` if the header is absent or the resulting address fails to parse.
pub fn get_forwarded_addr<B>(request: &Request<B>) -> Option<SocketAddr> {
    let forwarded_for = get_forwarded_for(request);
    if let Some(forwarded_for) = forwarded_for {
        let port = if let Some(remote_addr) = request.extensions().get::<ConnectInfo<SocketAddr>>()
        {
            remote_addr.port()
        } else {
            0
        };

        return format!("{forwarded_for}:{port}").parse().ok();
    }

    None
}

#[cfg(test)]
mod tests {
    use http::Request;

    use super::*;

    fn request_with_header(name: &str, value: &str) -> Request<()> {
        Request::builder().header(name, value).body(()).unwrap()
    }

    #[test]
    fn single_ip_is_returned() {
        let req = request_with_header("X-Forwarded-For", "1.2.3.4");
        assert_eq!(get_forwarded_for(&req), Some("1.2.3.4"));
    }

    #[test]
    fn first_ip_is_returned_from_chain() {
        let req = request_with_header("X-Forwarded-For", "1.2.3.4, 10.0.0.1, 192.168.1.1");
        assert_eq!(get_forwarded_for(&req), Some("1.2.3.4"));
    }

    #[test]
    fn missing_header_returns_none() {
        let req = Request::builder().body(()).unwrap();
        assert_eq!(get_forwarded_for(&req), None);
    }
}
