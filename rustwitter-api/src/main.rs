


static IP: &str = "127.0.0.1";
static PORT: &str = "8080";

#[async_std::main] 
async fn main() -> Result<(), std::io::Error> {

    let mut app = tide::new();
    app.at("/").get(|_| async move { Ok("Hello, world!") });

    println!("Listening at {}:{}", IP, PORT); 
    app.listen(format!("{}:{}", IP, PORT)).await?;
    Ok(())
}
