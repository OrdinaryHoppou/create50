pub mod app;
pub mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut app = app::App::new();
    app.run().await?;

    Ok(())
}
