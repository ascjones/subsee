use argh::FromArgs;
use color_eyre::eyre::{self, WrapErr};
use frame_metadata::v12::RuntimeMetadataPrefixed;

#[derive(FromArgs)]
/// Inspect substrate metadata
struct SubSee {
    /// the url of the substrate node to query for metadata
    #[argh(option, default = "String::from(\"http://localhost:9933\")")]
    url: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args: SubSee = argh::from_env();

    let metadata = fetch_metadata(&args.url)?;
    let serialized = serde_json::to_string_pretty(&metadata)?;

    println!("{}", serialized);
    Ok(())
}

fn fetch_metadata(url: &str) -> color_eyre::Result<RuntimeMetadataPrefixed<String>> {
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
