use reqwest::Error;
use serde_json;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let keys = Path::new("./keys.json");
    if !keys.exists() {
        let res: String = reqwest::get("http://0.0.0.0:8080/register_client")
            .await?
            .text()
            .await?;
        let mut keyfile = fs::File::create(keys).unwrap();
        keyfile.write(res.as_bytes()).unwrap();
    }
    loop {
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read user command.");

        match &cmd[..] {
            "hello\n" => {
                let res = reqwest::get("http://0.0.0.0:3000/").await?;

                let body = res.text().await?;
                println!("Request Body: {}", body);
            }
            _ => {
                println!("Please enter a valid command.");
            }
        }
    }

    Ok(())
}
