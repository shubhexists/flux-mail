use crate::database::DatabaseClient;
use std::error::Error;

pub struct Webhooks {
    mail: String,
    address: String,
    webhook_address: String,
}

impl Webhooks {
    pub async fn add_webhook_in_bulk(
        db: &DatabaseClient,
        records: Vec<Webhooks>,
    ) -> Result<(), Box<dyn Error>> {
        let sql = "
            INSERT INTO user_config (mail, address, web_hook_address)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
        ";

        for rec in records {
            db.db
                .execute(sql, &[&rec.mail, &rec.address, &rec.webhook_address])
                .await?;
        }
        Ok(())
    }

    pub async fn get_webhook_address_for_mail(
        db: &DatabaseClient,
        mail: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let row = db
            .db
            .query_opt(
                "SELECT web_hook_address FROM user_config WHERE mail = $1",
                &[&mail],
            )
            .await?;

        Ok(row.map(|r| r.get::<_, String>(0)))
    }

    pub async fn get_duplicate_mails(
        db: &DatabaseClient,
        mails: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let sql = "
            SELECT mail FROM user_config
            WHERE mail = ANY($1)
        ";

        let rows = db.db.query(sql, &[&mails]).await?;
        Ok(rows.into_iter().map(|r| r.get::<_, String>(0)).collect())
    }
}
