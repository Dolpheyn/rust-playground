use dotenv;

use serde_json::json;
use serde_json::Value;
use sqlx::{query, PgPool, Pool};
use std::env::var;
use tide::{http, Request, Response, Server};

#[cfg(test)]
mod tests;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    let app_url = var("APP_URL").unwrap();
    let app_port = var("APP_PORT").unwrap();

    let app: Server<State> = server().await;

    app.listen(format!("{}:{}", app_url, app_port)).await.unwrap();
}

async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move {
        let db_pool = &req.state().db_pool;
        let row = query!("select 'nadia' as cutie")
            .fetch_one(db_pool)
            .await
            .unwrap();

        let json = json!({
            "status": "Ok",
            "data": {
                "cutie": row.cutie,
            },
        });
        Ok(Response::new(http::StatusCode::Ok).body_json(&json)?)
    });

    //app.at("/users")
    //    .post(|mut req: Request<State>| async move {
    //        let db_pool = req.state().db_pool;
    //        let inputs: Value = req.body_json().await.unwrap();
    //        let username: &Value = inputs.get("username").unwrap();
    //        
    //        let query = query("insert into users (username) values (?)")
    //            .bind(username);

    //       //let result = db_pool.acquire().await.unwrap().execute(query);
    //        //Ok(Response::new(http::StatusCode::Ok).body_json(inputs).unwrap())
    //        Ok(Response::new(http::StatusCode::Ok).body_string(result.to_string()))
    //    });

    app
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}
