use crate::database::DatabaseClient;
use std::error::Error;

pub struct AddressLimits {
    pub address: String,
    pub limit: i64,
    pub completed: i64,
}

impl AddressLimits {
    pub async fn get_details_for_address(
        db: &DatabaseClient,
        address: &str,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let row = db
            .db
            .query_opt(
                "SELECT address, limit, completed FROM quota WHERE address = $1",
                &[&address],
            )
            .await?;

        Ok(row.map(|r| AddressLimits {
            address: r.get(0),
            limit: r.get(1),
            completed: r.get(2),
        }))
    }

    pub async fn increment(db: &DatabaseClient, address: &str) -> Result<(), Box<dyn Error>> {
        let sql = "
            UPDATE quota
            SET completed = completed + 1
            WHERE address = $1
        ";
        db.db.execute(sql, &[&address]).await?;
        Ok(())
    }
}
