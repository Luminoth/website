#![deny(warnings)]

mod handlers;
mod models;
mod options;
mod routes;
mod util;

use std::net::SocketAddr;

use structopt::StructOpt;
use warp::Filter;

use crate::options::Options;
use crate::util::OptFmt;

// TODO: make this configurable
pub const REGION: &str = "us-west-2";

fn init_logging() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::from_args();

    init_logging();

    let routes = routes::init_routes().with(routes::init_cors());

    let addr = options
        .address()
        .parse::<SocketAddr>()
        .expect("Invalid address");

    let filter = routes.with(warp::log::custom(|info| {
        // ignore AWS health check requests
        if let Some(user_agent) = info.user_agent() {
            if user_agent.contains("HealthChecker") {
                return;
            }
        }

        // check for forwarding
        let mut forwarded = true;
        let mut remote_addr = util::forwarded_addr(&info);
        if remote_addr.is_none() {
            remote_addr = info.remote_addr();
            forwarded = false;
        }

        log::info!(
            target: "energonsoftware::api",
            "{}{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
            OptFmt(remote_addr),
            if forwarded { " (forwarded)" } else { "" },
            info.method(),
            info.path(),
            info.version(),
            info.status().as_u16(),
            OptFmt(info.referer()),
            OptFmt(info.user_agent()),
            info.elapsed(),
        );
    }));
    warp::serve(filter).run(addr).await;

    Ok(())
}
