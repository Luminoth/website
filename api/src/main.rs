#![deny(warnings)]

mod handlers;
mod models;
mod options;
mod routes;
mod util;

use std::net::SocketAddr;
use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use warp::Filter;

use crate::options::SharedOptions;
use crate::util::OptFmt;

static OPTIONS: Lazy<SharedOptions> = Lazy::new(|| Arc::new(RwLock::new(argh::from_env())));

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    // TODO: make this configurable
    let region = String::from("us-west-2");

    let addr = OPTIONS
        .read()
        .address()
        .parse::<SocketAddr>()
        .expect("Invalid address");

    let routes =
        routes::init_routes(region, OPTIONS.clone()).with(routes::init_cors(!OPTIONS.read().prod));
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

        info!(
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
