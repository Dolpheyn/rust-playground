#[macro_use]
extern crate rocket;

use rocket::tokio::{
    task::spawn_blocking,
    time::{sleep, Duration},
};
use std::io;

// A handler function is a standalone fn with a `route` attribute.
// Route attribute = `#[get()]`, `#[post()]` and other HTTP methods.
// https://api.rocket.rs/v0.5-rc/rocket/attr.route.html
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Can bind a value from request to handlers' parameter.
// The value will be passed as an argument to the handler.
// All the dynamic parameters must appear in the function's parameter with the same name.
//
// Any types needs to implement/derive the `FromParam` trait. The trait converts a dynamic path
// segment string to a concrete value.
// https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromParam.html
//
// Other std types that has `FromParam` implemented:
// https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromParam.html#provided-implementations
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} fucking seconds innit bruv", seconds)
}

// Self defined type need to derive `FromForm` to be able to use like `/path?<ident>` or
// `/path?<ident..>`.
#[derive(FromForm)]
struct TimeWrapper {
    seconds: u64,
}

// The type in a handler fn's parameters can be self defined types too.
// The fields in the struct corresponds to the query in url
// http://127.0.0.1?/delay_struct?seconds=5
//
// Note: Can also use request data by implementing the `FromData` trait.
// https://api.rocket.rs/v0.5-rc/rocket/data/trait.FromData.html
#[get("/delay_struct?<outer..>")]
async fn delay_struct(outer: TimeWrapper) -> String {
    sleep(Duration::from_secs(outer.seconds)).await;
    format!("Waited for {} fucking seconds innit bruv", outer.seconds)
}

// Handlers can be async too.
//
// Can use `rocket::tokio::task::spawn_blocking` to turn sync(blocking) operations into an async
// one so you can `.await` in an `async fn`.
#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

// `'r` means that the value will live as long as the request live, because it is stored in the
// request's local cache by rocket.
struct KeyValue<'r> {
    key: &'r str,
    value: usize,
}

use rocket::request::FromParam;

// Custom `FromParam` implementation that parses "key:value" in parameter into
// `KeyValue {key, value}`
impl<'r> FromParam<'r> for KeyValue<'r> {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let (key, val_str) = match param.find(':') {
            Some(i) if i > 0 => (&param[..i], &param[(i + 1)..]),
            _ => return Err("Need key:value".to_string()),
        };

        if !key.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Ascii only bruv don't play with man's patience innit".to_string());
        }

        val_str
            .parse()
            .map(|value| KeyValue { key, value })
            .map_err(|e| format!("Error in parsing value '{}': {}", val_str, e.to_string()))
    }
}

#[get("/<kv>")]
fn key_val(kv: Result<KeyValue, String>) -> String {
    match kv {
        Ok(KeyValue { key, value }) => format!("The key to the value {} is {}", value, key),
        Err(e) => e,
    }
}

#[launch]
fn rocket() -> _ {
    // create an instance of Rocket and mount `/`, `/delay/<seconds>`, `/delay?seconds=` and
    // `/task/blocking_task`.
    rocket::build()
        .mount("/", routes![index, delay, delay_struct])
        .mount("/kv", routes![key_val])
        .mount("/task", routes![blocking_task])
}
