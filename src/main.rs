use argh::FromArgs;
use color_eyre::eyre::{self, WrapErr};
use frame_metadata::{
    decode_different::{DecodeDifferent, DecodeDifferentArray},
    RuntimeMetadata, RuntimeMetadataPrefixed,
};
use std::io::{self, Write};

#[derive(FromArgs)]
/// Inspect substrate metadata
struct SubSee {
    /// the url of the substrate node to query for metadata
    #[argh(option, default = "String::from(\"http://localhost:9933\")")]
    url: String,
    /// the name of a pallet to display metadata for, otherwise displays all
    #[argh(option, short = 'p')]
    pallet: Option<String>,
    /// the format of the metadata to display: `json`, `hex` or `bytes`
    #[argh(option, short = 'f', default = "\"json\".to_string()")]
    format: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: SubSee = argh::from_env();

    let json = fetch_metadata(&args.url)?;
    let hex_data = json["result"]
        .as_str()
        .ok_or(eyre::eyre!("metadata result field should be a string"))?;
    let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;

    match args.format.as_str() {
        "json" => {
            let metadata = scale::Decode::decode(&mut &bytes[..])?;
            display_metadata_json(metadata, &args)
        }
        "hex" => {
            println!("{}", hex_data);
            Ok(())
        }
        "bytes" => Ok(io::stdout().write_all(&bytes)?),
        _ => Err(eyre::eyre!(
            "Unsupported format `{}`, expected `json`, `hex` or `bytes`",
            args.format
        )),
    }
}

fn fetch_metadata(url: &str) -> color_eyre::Result<serde_json::Value> {
    let resp = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "state_getMetadata",
            "id": 1
        }))
        .context("error fetching metadata from the substrate node")?;

    Ok(resp.into_json()?)
}

fn display_metadata_json(
    metadata: RuntimeMetadataPrefixed,
    args: &SubSee,
) -> color_eyre::Result<()> {
    let serialized = if let Some(ref pallet) = args.pallet {
        match metadata.1 {
            RuntimeMetadata::V12(v12) => {
                let modules = match v12.modules {
                    DecodeDifferentArray::Decoded(modules) => modules,
                    DecodeDifferentArray::Encode(_) => {
                        return Err(eyre::eyre!("Metadata should be Decoded"))
                    }
                };
                let module = modules
                    .iter()
                    .find(|module| module.name == DecodeDifferent::Decoded(pallet.into()))
                    .ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
                serde_json::to_string_pretty(&module)?
            }
            RuntimeMetadata::V13(v13) => {
                let modules = match v13.modules {
                    DecodeDifferentArray::Decoded(modules) => modules,
                    DecodeDifferentArray::Encode(_) => {
                        return Err(eyre::eyre!("Metadata should be Decoded"))
                    }
                };
                let module = modules
                    .iter()
                    .find(|module| module.name == DecodeDifferent::Decoded(pallet.into()))
                    .ok_or_else(|| eyre::eyre!("pallet not found in metadata"))?;
                serde_json::to_string_pretty(&module)?
            }
            RuntimeMetadata::V14(v14) => {
                let pallet = v14
                    .pallets
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
