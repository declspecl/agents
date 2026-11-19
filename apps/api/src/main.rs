use lambda_http::{
    Error,
    run,
    tracing,
};

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    if std::env::var_os("AWS_LAMBDA_RUNTIME_API").is_some() {
        run(routes::router()).await
    } else {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        println!("listening on http://localhost:3000");

        axum::serve(listener, routes::router()).await?;

        Ok(())
    }
}
