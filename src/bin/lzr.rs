#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lazyarchbuild::run().await
}
