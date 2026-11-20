use lambda_http::Error;

use crate::environment::{
    Environment,
    Runtime,
};
use crate::routes;

pub async fn serve(environment: Environment) -> Result<(), Error> {
    let router = routes::build_router();

    match environment.runtime {
        Runtime::Lambda => lambda_http::run(router).await,
        Runtime::Local {
            bind_address,
        } => {
            let listener = tokio::net::TcpListener::bind(bind_address).await?;
            println!("listening on http://{bind_address}");

            axum::serve(listener, router).await?;

            Ok(())
        },
    }
}
