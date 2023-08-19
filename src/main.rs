use clap::{Parser, Subcommand};
use starknet::providers::SequencerGatewayProvider;

use rifle::{blocknumber_to_timestamp, timestamp_to_blocknumber, timestamp_to_unix};

#[derive(Debug, Parser)]
struct Cli {
    // The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    // #[arg(short, long)]
    // #[arg(value_name = "TIME_ZONE")]
    // #[arg(help = "Timezone")]
    // timezone: Option<String>,
    #[arg(short, long)]
    #[arg(value_name = "NETWORK")]
    #[arg(help = "Network: [mainnet/goerli/goerli2]")]
    network: Option<String>,

    // #[arg(short = 'u', long)]
    // #[arg(value_name = "RPC_URL")]
    // #[arg(help = "The RPC endpoint")]
    // rpc_url: Option<String>,
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
        time: u64,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let mut provider = SequencerGatewayProvider::starknet_alpha_mainnet();
    match args.network {
        Some(network) => {
            if network == "goerli" {
                println!("{}", "Use goerli network as provider");
                provider = SequencerGatewayProvider::starknet_alpha_goerli();
            } else if network == "goerli2" {
                println!("{}", "Use goerli2 network as provider");
                provider = SequencerGatewayProvider::starknet_alpha_goerli_2();
            }
        }
        None => {
            println!("{}", "Default network is mainnet");
        }
    }

    // match args.rpc_url {
    //     Some(url) => {
    //         let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(&url).unwrap()));
    //         let time_result = blocknumber_to_timestamp(&rpc_client, block_number).await;
    //     }
    //     None => {}
    // }

    match args.command {
        Some(Commands::BlockToTime { block_number }) => {
            let time_result = blocknumber_to_timestamp(&provider, block_number).await;
            match time_result {
                Ok(time_result) => match args.format {
                    Some(format) => {
                        if format == "unix" {
                            println!("{}", timestamp_to_unix(time_result));
                        } else {
                            println!("{}", "Unvalid Format");
                            println!("{}", time_result);
                        }
                    }
                    None => {
                        println!(
                            "Blocknumber #{} is created at {}.",
                            block_number, time_result
                        );
                    }
                },
                Err(err) => println!("{}", err),
            }
        }
        Some(Commands::TimeToBlock { time }) => {
            let block_result = timestamp_to_blocknumber(&provider, time).await;
            match block_result {
                Ok(block_result) => {
                    println!(
                        "Most closest block that created at time {} is: #{}",
                        time, block_result
                    );
                }
                Err(err) => println!("{}", err),
            }
        }
        None => {}
    }
}
