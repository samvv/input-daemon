use std::future::pending;

use input_daemon::server::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Service::new()?;
    let _conn = zbus::connection::Builder::system()?
        .name("be.samvervaeck.InputDaemon")?
        .serve_at("/be/samvervaeck/InputDaemon", service)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
