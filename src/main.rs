use subxt;
use tokio;
use subxt::{ClientBuilder, DefaultConfig, SubstrateExtrinsicParams};
use sp_keyring::AccountKeyring;
use subxt::PairSigner;

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod node_runtime {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
    .set_url("http://localhost:9933/")
    .build()
    .await?
    .to_runtime_api::<node_runtime::RuntimeApi<DefaultConfig, SubstrateExtrinsicParams<DefaultConfig>>>();

    Ok(())
}
