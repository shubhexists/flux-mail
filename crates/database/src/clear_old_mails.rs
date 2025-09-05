use crate::database::DatabaseClient;
use std::error::Error;

pub fn clear_old_mails(period: tokio::time::Duration) {
    std::thread::spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .map_err(|e: std::io::Error| format!("Failed to build async runtime: {}", e))?;

        runtime.block_on(async move {
            let local: tokio::task::LocalSet = tokio::task::LocalSet::new();
            local.spawn_local(async move {
                let db: DatabaseClient = match DatabaseClient::connect().await {
                    Ok(db) => db,
                    Err(e) => {
                        tracing::error!("Failed to connect to database: {}", e);
                        return;
                    }
                };
                let mut interval: tokio::time::Interval = tokio::time::interval(period);
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                loop {
                    interval.tick().await;
                    if let Err(e) = db.delete_old_mail().await {
                        tracing::error!("Failed to delete old mail: {}", e);
                    }
                }
            });
            local.await;
        });
        Ok(())
    });
}
