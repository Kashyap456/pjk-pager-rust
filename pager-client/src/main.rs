use reqwest::Error;
use std::io;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
