use color_eyre::eyre::eyre;
use frame_metadata::v12::RuntimeMetadataPrefixed;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    // todo: make CLI args
    let url = "http://localhost:9933";

    let metadata = fetch_metadata(&url)?;
    let decoded: RuntimeMetadataPrefixed<String> = scale::Decode::decode(&mut &metadata[..])?;
    let serialized = serde_json::to_string_pretty(&decoded)?;
    println!("{}", serialized);

    Ok(())
}

fn fetch_metadata(url: &str) -> color_eyre::Result<Vec<u8>> {
    let resp = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
              "jsonrpc": "2.0",
              "method": "state_getMetadata",
              "id": 1
          }))?;
    let json: serde_json::Value = resp.into_json()?;
    let hex_data = json["result"]
        .as_str()
        .ok_or(eyre!("metadata result field should be a string"))?;
    let bytes = hex::decode(hex_data.trim_start_matches("0x"))?;
    Ok(bytes)
}
