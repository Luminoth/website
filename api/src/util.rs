use std::fmt;
use std::net::SocketAddr;

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

fn forwarded_for<'a>(info: &'a warp::log::Info) -> Option<&'a str> {
    let forwarded_for = info.request_headers().get("x-forwarded-for");
    if let Some(forwarded_for) = forwarded_for {
        if let Ok(forwarded_for) = forwarded_for.to_str() {
            let addrs = forwarded_for.split(',').collect::<Vec<&str>>();
            if !addrs.is_empty() {
                return Some(addrs[0]);
            }
        }
    }

    None
}

pub fn forwarded_addr(info: &warp::log::Info) -> Option<SocketAddr> {
    let forwarded_for = forwarded_for(info);
    if let Some(forwarded_for) = forwarded_for {
        let port = if let Some(remote_addr) = info.remote_addr() {
            remote_addr.port()
        } else {
            0
        };

        return format!("{}:{}", forwarded_for, port).parse().ok();
    }

    None
}
