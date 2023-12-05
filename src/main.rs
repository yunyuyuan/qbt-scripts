use std::collections::HashMap;

use clap::Parser;
use reqwest::*;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    username: String,

    #[arg(long)]
    password: String,

    #[arg(long)]
    url: String,

    #[arg(long)]
    hash: String,
}

fn main() {
    let args = Args::parse();
    
    let base_url = Url::parse(&args.url).expect("Error parsing url");
    let get_url = |path: &str| -> Url {
        base_url.join(path).expect("")
    };
    
    let client = blocking::Client::builder().cookie_store(true).build().expect("");

    let mut map = HashMap::new();
    map.insert("username", args.username);
    map.insert("password", args.password);
    client.post(get_url("/api/v2/auth/login"))
        .form(&map)
        .send().expect("Error when login");
    
    let mut map = HashMap::new();
    map.insert("hashes", args.hash);
    map.insert("ratioLimit", String::from("2.00"));
    map.insert("seedingTimeLimit", String::from("-1"));
    map.insert("inactiveSeedingTimeLimit", String::from("-1"));
    client.post(get_url("/api/v2/torrents/setShareLimits"))
        .form(&map)
        .send().expect("Error when set limits");
}
