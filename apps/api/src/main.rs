use anyhow::Context;
use api::content::Catalog;
use api::environment::Environment;
use api::server;
use lambda_http::tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing::init_default_subscriber();

    let environment = Environment::read().context("could not read the environment")?;
    let catalog = Catalog::load_embedded().context("could not load the embedded content")?;

    server::serve(environment, catalog).await
}
