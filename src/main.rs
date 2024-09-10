use clap::{Parser, Subcommand};
use dotenv::dotenv;
use notion_client::{endpoints::Client, NotionClientError};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Args::parse();

    let notion_token: String = env::var("NOTION_TOKEN").expect("NOTION_TOKEN must be set");
    let notion = Notion::new(notion_token).unwrap();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Block { id }) => {
            let response = notion.fetch_block_as_json(id).await.unwrap();
            println!("{}", response);
        }
        None => {}
    }
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Block stuff
    Block {
        /// retrieve a block by its ID
        #[arg(short, long)]
        id: String,
    },
}

pub struct Notion {
    client: Client,
}

impl Notion {
    pub fn new(token: String) -> Result<Self, NotionClientError> {
        let client = Client::new(token, None);
        match client {
            Ok(c) => Ok(Notion { client: c }),
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_block_as_json(&self, block_id: &str) -> Result<String, NotionClientError> {
        let block = self.client.blocks.retrieve_a_block(block_id).await?;

        match serde_json::to_string_pretty(&block) {
            Ok(json) => Ok(json),
            Err(e) => Err(NotionClientError::FailedToSerialize {
                source: serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                )),
            }),
        }
    }
}
