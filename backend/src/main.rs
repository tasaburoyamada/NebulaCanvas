#[tokio::main]
async fn main() -> anyhow::Result<()> {
    nebula_backend::start_backend().await
}
