#[tokio::main(flavor = "current_thread")]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    println!("Hello, world!");

    tracing::info!("Rust is the future!");

    Ok(())
}
