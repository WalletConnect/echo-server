use {dotenv::dotenv, echo_server::env, tokio::sync::broadcast};

#[tokio::main]
async fn main() -> echo_server::error::Result<()> {
    let (_signal, shutdown) = broadcast::channel(1);
    dotenv().ok();
    let config =
        env::get_config().expect("Failed to load config, please ensure all env vars are defined.");
    echo_server::bootstap(shutdown, config).await
}
