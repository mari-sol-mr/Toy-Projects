use chrono::NaiveDateTime;
#[derive(Debug, sqlx::FromRow)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub imei: String,
    pub changed_parts: Vec<String>,
    pub created_ts: NaiveDateTime,
}

pub async fn create_record(pool: &sqlx::PgPool, name: &str, imei: &str, changed_parts:  Vec<String>) -> Result<(), sqlx::Error> {
    let timestamp = chrono::Local::now().naive_local();
    sqlx::query(
        "INSERT INTO tutorial_table (name, imei, changed_parts, created_ts) VALUES ($1, $2, $3, $4)"
    )
    .bind(name)
    .bind(imei)
    .bind(changed_parts)
    .bind(timestamp)
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_record(pool: &sqlx::PgPool, imei: &str) -> Result<Option<Record>, sqlx::Error> {
    let record = sqlx::query_as::<_, Record>(
        "SELECT id, name, imei, changed_parts, created_ts FROM tutorial_table WHERE imei = $1"
    )
    .bind(imei)
    .fetch_optional(pool)
    .await?;
    
    Ok(record)
}

pub async fn get_latest_records(pool: &sqlx::PgPool, limit: i32) -> Result<Vec<Record>, sqlx::Error> {
    let users = sqlx::query_as::<_, Record>(
        "SELECT id, name, imei, changed_parts, created_ts
         FROM tutorial_table 
         ORDER BY created_ts DESC 
         LIMIT $1"
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}

pub async fn delete_imei(pool: &sqlx::PgPool, imei: &str) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tutorial_table WHERE imei = $1")
        .bind(imei)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}

pub async fn update_imei_by_imei(
    pool: &sqlx::PgPool,
    old_imei: &str,
    new_imei: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("UPDATE tutorial_table SET imei = $1 WHERE imei = $2")
        .bind(new_imei)
        .bind(old_imei)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}