use argh::FromArgs;
use color_eyre::eyre::{self, WrapErr};
use frame_metadata::{v12, RuntimeMetadata, RuntimeMetadataPrefixed};

#[derive(FromArgs)]
/// Inspect substrate metadata
struct SubSee {
    /// the url of the substrate node to query for metadata
    #[argh(option, default = "String::from(\"http://localhost:9933\")")]
    url: String,
    /// the name of a pallet to display metadata for, otherwise displays all
    #[argh(option, short = 'p')]
    pallet: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: SubSee = argh::from_env();

    let metadata = fetch_metadata(&args.url)?;
    display_metadata(metadata, &args)?;

    Ok(())
}

fn fetch_metadata(url: &str) -> color_eyre::Result<RuntimeMetadataPrefixed> {
    let resp = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "state_getMetadata",
            "id": 1
        }))
        .context("error fetching metadata from the substrate node")?;

    let json: serde_json::Value = resp.into_json()?;
    let hex_data = json["result"]
        .as_str()
        .ok_or(eyre::eyre!("metadata result field should be a string"))?;

    let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;
    let decoded = scale::Decode::decode(&mut &bytes[..])?;
    Ok(decoded)
}

fn display_metadata(
    metadata: RuntimeMetadataPrefixed,
    args: &SubSee,
) -> color_eyre::Result<()> {
    let serialized = if let Some(ref pallet) = args.pallet {
        match metadata.1 {
            RuntimeMetadata::V12(v12) => {
                let modules = match v12.modules {
                    v12::DecodeDifferentArray::Decoded(modules) => modules,
                    v12::DecodeDifferentArray::Encode(_) => {
                        return Err(eyre::eyre!("Metadata should be Decoded"))
                    }
                };
                let pallet_metadata = modules
                    .iter()
                    .find(|module| module.name == v12::DecodeDifferent::Decoded(pallet.into()))
                    .ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
                serde_json::to_string_pretty(&pallet_metadata)?
            }
            RuntimeMetadata::V13(v13) => {
                let pallet = v13
                    .modules
                    .iter()
                    .find(|m| &m.name == pallet)
                    .ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
                serde_json::to_string_pretty(&pallet)?
            }
            _ => return Err(eyre::eyre!("Unsupported metadata version")),
        }
    } else {
        serde_json::to_string_pretty(&metadata)?
    };
    println!("{}", serialized);
    Ok(())
}
