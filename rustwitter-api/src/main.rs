use dotenv;
use std::{ env::var };
use sqlx::{ Pool, PgPool, query };
use tide::{ Server, Request };

#[async_std::main] 
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = var("DATABASE_URL").unwrap();
    let app_url = var("APP_URL").unwrap();
    let app_port = var("APP_PORT").unwrap();

    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    let mut app: Server<State> = tide::with_state(State {
	db_pool,
    });

    app.at("/").get(|req: Request<State>| async move {
	let db_pool = &req.state().db_pool;
	let row = query!("select 'nadia' as cutie")
	    .fetch_one(db_pool)
	    .await.unwrap();

	Ok(format!("{}", row.cutie.unwrap()))
    });
    app.listen(format!("{}:{}", app_url, app_port)).await.unwrap();
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}
