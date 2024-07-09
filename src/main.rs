use anyhow::Ok;
use sqlx::postgres::PgPool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    let id = add_todo(&pool, "Writing Code".to_string()).await?;
    println!("Added todo with id: {}", id);

    let completed = complete_todo(&pool, id).await?;
    println!("task: {} is completed: {}", id, completed);

    list_todos(&pool).await?;

    Ok(())
}

async fn add_todo(pool: &PgPool, description: String) -> anyhow::Result<i64> {
    let rec = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( $1 )
RETURNING id
        "#,
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

async fn complete_todo(pool: &PgPool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &PgPool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}
