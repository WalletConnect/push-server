#[tokio::main]
async fn main() -> echo_server::error::Result<()> {
    Ok(echo_server::bootstap().await?)
}
