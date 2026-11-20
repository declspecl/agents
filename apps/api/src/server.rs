//! Serves the router the way the detected runtime hosts it.

use anyhow::Context;

use crate::content::Catalog;
use crate::environment::{
    Environment,
    Runtime,
};
use crate::routes;
use crate::state::AppState;

/// Runs until the process is stopped, or until the host fails.
pub async fn serve(
    environment: Environment,
    catalog: Catalog,
) -> anyhow::Result<()> {
    let router = routes::build_router(AppState::new(catalog, environment.base_url));

    match environment.runtime {
        Runtime::Lambda => lambda_http::run(router).await.map_err(anyhow::Error::from_boxed).context("the Lambda runtime failed"),
        Runtime::Local {
            bind_address,
        } => {
            let listener = tokio::net::TcpListener::bind(bind_address).await.with_context(|| format!("could not listen on {bind_address}"))?;
            println!("listening on http://{bind_address}");
            axum::serve(listener, router).await.context("the server failed")
        },
    }
}
