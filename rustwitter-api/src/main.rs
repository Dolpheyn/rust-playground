use dotenv;
use std::{ env::var };

#[async_std::main] 
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = var("DATABASE_URL").unwrap();
    let app_url = var("APP_URL").unwrap();
    let app_port = var("APP_PORT").unwrap();
    dbg!(db_url);

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello, world!") });

    app.listen(format!("{}:{}", app_url, app_port)).await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
