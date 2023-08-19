use clap::{Parser, Subcommand};
use woodcock::{blocknumber_to_timestamp, timestamp_to_blocknumber};

#[derive(Debug, Parser)]
struct Cli {
    // The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    #[arg(value_name = "TIME_ZONE")]
    #[arg(help = "Timezone")]
    timezone: Option<String>,

    #[arg(short = 'u', long)]
    #[arg(env = "STARKNET_RPC_URL")]
    #[arg(help = "The RPC endpoint")]
    rpc_url: String,

    #[arg(short, long)]
    #[arg(value_name = "TIME_FORMAT")]
    #[arg(help = "The format to use time")]
    format: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// blocknumber to time
    #[command(visible_alias = "btt")]
    #[command(name = "--to-time")]
    #[command(about = "Convert blocknumber to time.")]
    BlockToTime {
        /// The blocknumber to convert
        #[arg(required = true)]
        block_number: u64,
    },

    /// time to blocknumber
    #[command(visible_alias = "ttb")]
    #[command(name = "--to-block")]
    #[command(about = "Convert time to blocknumber.")]
    TimeToBlock {
        /// The time to convert
        #[arg(required = true)]
        time: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);

    match args.command {
        Some(Commands::BlockToTime { block_number }) => {
            let time_result = blocknumber_to_timestamp(block_number).await;
            println!("{}", time_result.unwrap());
        }
        Some(Commands::TimeToBlock { time }) => {
            let block_result = timestamp_to_blocknumber(time).await;
            println!("{}", block_result.unwrap());
        }
        None => {}
    }
}
