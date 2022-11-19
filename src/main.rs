use dotenv::dotenv;
use echo_server::env;

#[tokio::main]
async fn main() -> echo_server::error::Result<()> {
    dotenv().ok();
    let config = env::get_config()
        .expect("Failed to load config, please ensure all env vars are defined.");
    Ok(echo_server::bootstap(config).await?)
}
