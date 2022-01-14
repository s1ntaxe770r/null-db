use std::collections::HashMap;
use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "nulldb")]
#[clap(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Put {
        key: String,
        value: String,
        #[clap(long, default_value = "localhost")]
        host: String,
    },

    Get { 
        key: String,
        #[clap(long, default_value = "localhost")]
        host: String,
    }

}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "localhost")]
    host: String,
    #[clap(short, long, default_value = "data")]
    data: String,
    #[clap(short, long, default_value = "key")]
    key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    match &args.command {
        Commands::Put { key, value, host } => {
            println!("putting data {}", value);
            let client = reqwest::Client::new();
            let data = value.clone();
            let _res = client.post(format!("http://{}/{}\n",host, key))
                .body(data)
                .send()
                .await?;
        }

        Commands::Get { key , host} => {
            println!("getting data for key {}", key);
            let _resp = reqwest::get(format!("http://{}/{}\n",host, key))
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        }
    }

    Ok(())
}
