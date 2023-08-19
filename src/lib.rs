use anyhow::Result;
use chrono::prelude::*;
use starknet::core::types::MaybePendingBlockWithTxHashes::{Block, PendingBlock};
use starknet::providers::ProviderError::{ArrayLengthMismatch, Other, RateLimited, StarknetError};
use starknet::{
    core::types::BlockId,
    providers::{Provider, SequencerGatewayProvider},
};

pub async fn blocknumber_to_timestamp(
    provider: &SequencerGatewayProvider,
    blocknumber: u64,
) -> Result<u64> {
    let current_timstamp = get_block_timestamp(&provider, blocknumber).await?;
    Ok(current_timstamp)
}

pub async fn timestamp_to_blocknumber(
    provider: &SequencerGatewayProvider,
    timestamp: u64,
) -> Result<u64> {
    let blocknumber = block_search(&provider, timestamp).await?;
    Ok(blocknumber)
}

// refered from https://github.com/0xcacti/snipe
async fn block_search(provider: &SequencerGatewayProvider, target_timestamp: u64) -> Result<u64> {
    let current_blocknumber = get_current_block_number(&provider).await?;
    let current_timstamp = get_block_timestamp(&provider, current_blocknumber).await?;

    let genesis_timestamp = get_block_timestamp(&provider, 0).await?;
    if target_timestamp < genesis_timestamp {
        return Err(anyhow::anyhow!(
            "Time is before genesis block's time: {}/{}",
            timestamp_to_unix(genesis_timestamp),
            genesis_timestamp
        ));
    }
    if target_timestamp > current_timstamp {
        return Err(anyhow::anyhow!(
            "Time is in the future, Latest time is:{}/{}",
            timestamp_to_unix(current_timstamp),
            current_timstamp
        ));
    }

    let mut lower_bound = 0;
    let mut upper_bound = current_blocknumber;
    let mut current_blocknumber = current_blocknumber / 2;
    let mut current_timstamp = get_block_timestamp(&provider, current_blocknumber).await?;

    while lower_bound <= upper_bound {
        if current_timstamp == target_timestamp {
            return Ok(current_blocknumber);
        }

        if current_timstamp < target_timestamp {
            lower_bound = current_blocknumber + 1;
        } else {
            upper_bound = current_blocknumber - 1;
        }
        current_blocknumber = (lower_bound + upper_bound) / 2;
        current_timstamp = get_block_timestamp(&provider, current_blocknumber).await?;
    }
    if lower_bound > upper_bound {
        current_blocknumber = lower_bound;
    }
    Ok(current_blocknumber)
}

async fn get_block_timestamp(
    provider: &SequencerGatewayProvider,
    block_number: u64,
) -> Result<u64> {
    let block = provider
        .get_block_with_tx_hashes(BlockId::Number(block_number))
        .await;

    match block {
        Ok(full_block) => match full_block {
            Block(finalized_block) => {
                let timestamp = finalized_block.timestamp;
                Ok(timestamp)
            }
            PendingBlock(_) => Err(anyhow::anyhow!("No pending block in starknet")),
        },
        Err(err) => match err {
            StarknetError(e) => {
                println!("{:#?}", e.message);
                Err(anyhow::anyhow!(e.message))
            }
            RateLimited => {
                println!("{:#?}", "Rate Limited");
                Err(anyhow::anyhow!("RateLimited"))
            }
            ArrayLengthMismatch => {
                println!("{:#?}", "Array Length mismatch");
                Err(anyhow::anyhow!("ArrayLengthMismatch"))
            }
            Other(_) => {
                println!("{:#?}", "othererror");
                Err(anyhow::anyhow!("Other Error"))
            }
        },
    }
}

async fn get_current_block_number(provider: &SequencerGatewayProvider) -> Result<u64> {
    let block_number = provider.block_number().await?;
    Ok(block_number)
}

pub fn timestamp_to_unix(timestamp: u64) -> DateTime<Utc> {
    let date_time = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
    date_time
}
