#![deny(warnings)]

mod handlers;
mod models;
mod options;
mod routes;
mod state;
mod util;

use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use parking_lot::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use warp::Filter;

use crate::options::Options;
use crate::state::AppState;
use crate::util::OptFmt;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    // TODO: make this not mutually exclusive
    if options.tracing {
        console_subscriber::init();
    } else {
        init_logging()?
    };

    // TODO: make region configurable?
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region("us-west-2")
        .load()
        .await;
    let app_state = AppState::new(aws_config);

    let addr = options
        .address()
        .parse::<SocketAddr>()
        .unwrap_or_else(|_| panic!("Invalid address: {}", options.address()));

    let options = Arc::new(RwLock::new(options));

    let routes = routes::init_routes(app_state, options.clone())
        .with(routes::init_cors(!options.read().prod));
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
