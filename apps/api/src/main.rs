use std::process::ExitCode;

use api::environment::Environment;
use api::server;
use lambda_http::tracing;

#[tokio::main]
async fn main() -> ExitCode {
    tracing::init_default_subscriber();

    let environment = match Environment::read() {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");

            return ExitCode::FAILURE;
        },
    };

    match server::serve(environment).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
    }
}
