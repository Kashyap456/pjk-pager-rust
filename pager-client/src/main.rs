use futures_util::{future, pin_mut, SinkExt, StreamExt};
use reqwest::header::AUTHORIZATION;
use reqwest::Error;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

#[derive(Deserialize)]
struct Auth {
    access_token: String,
    refresh_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let keys = Path::new("./keys.json");
    if !keys.exists() {
        let res: String = reqwest::get("http://0.0.0.0:8080/register_client")
            .await?
            .text()
            .await?;
        let mut keyfile = fs::File::create(keys).unwrap();
        keyfile.write(res.as_bytes()).unwrap();
    }

    let mut auth: Option<Auth> = None;
    let mut username: Option<String> = None;
    let mut stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>> = None;

    loop {
        let mut cmd = String::new();

        if let Some(prompt) = username.as_ref() {
            eprint!("{}: ", prompt);
        }

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read user command.");

        let cmd_vec: Vec<&str> = cmd.split_whitespace().collect();

        match &cmd_vec[0][..] {
            "hello" => {
                let res = reqwest::get("http://0.0.0.0:3000/").await?;

                let body = res.text().await?;
                println!("Request Body: {}", body);
            }
            "login" => {
                println!("{} {}", &cmd_vec[1], &cmd_vec[2]);
                let mut map = HashMap::new();
                map.insert("username", &cmd_vec[1]);
                map.insert("password", &cmd_vec[2]);
                let res = client
                    .get("http://0.0.0.0:8080/login_user")
                    .json(&map)
                    .send()
                    .await?;

                auth = Some(res.json::<Auth>().await.unwrap());
                username = Some(cmd_vec[1].to_owned());
                let url = url::Url::parse("ws://0.0.0.0:7777/ws").unwrap();
                let (localstream, _) = connect_async(url).await.unwrap();
                let (write, read) = localstream.split();
                //stream = Some(localstream);
            }
            "register" => {
                let mut map = HashMap::new();
                map.insert("username", cmd_vec[1]);
                map.insert("password", cmd_vec[2]);
                let res = client
                    .post("http://0.0.0.0:8080/register_user")
                    .json(&map)
                    .send()
                    .await?;
            }
            "create_group" => {
                if username.is_none() {
                    eprintln!("Please login before using this command.");
                } else {
                    let user = username.as_ref().unwrap().as_str();
                    let mut map = HashMap::new();
                    map.insert("name", cmd_vec[1]);
                    map.insert("user", user);
                    let res = client
                        .post("http://0.0.0.0:3000/groups")
                        .header(
                            AUTHORIZATION,
                            "Bearer ".to_owned() + auth.as_ref().unwrap().access_token.as_str(),
                        )
                        .json(&map)
                        .send()
                        .await?;
                }
            }
            "join_group" => {
                if username.is_none() {
                    eprintln!("Please login before using this command.");
                } else {
                    let user = username.as_ref().unwrap().as_str();
                    let mut map = HashMap::new();
                    map.insert("name", cmd_vec[1]);
                    map.insert("user", user);
                    let res = client
                        .post("http://0.0.0.0:3000/memberships")
                        .header(
                            AUTHORIZATION,
                            "Bearer ".to_owned() + auth.as_ref().unwrap().access_token.as_str(),
                        )
                        .json(&map)
                        .send()
                        .await?;
                }
            }
            _ => {
                println!("Please enter a valid command.");
            }
        }
    }

    Ok(())
}
