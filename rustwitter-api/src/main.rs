use dotenv;
use std::{ env::var };
use sqlx::{ Pool, PgPool, query };
use tide::{ Server, Request };

#[async_std::main] 
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = var("DATABASE_URL")?;
    let app_url = var("APP_URL")?;
    let app_port = var("APP_PORT")?;

    let db_pool: PgPool = Pool::new(&db_url).await?;

    let mut app: Server<State> = tide::with_state(State {
	db_pool,
    });

    app.at("/").get(|req: Request<State>| async move {
	let db_pool = &req.state().db_pool;
	let row = query!("select 'nadia' as cutie")
	    .fetch_one(db_pool)
	    .await?;

	Ok(format!("{}", row.cutie.unwrap()))
    });
    app.listen(format!("{}:{}", app_url, app_port)).await?;

    Ok(())
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
