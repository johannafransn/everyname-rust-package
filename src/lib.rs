
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let name = "everyname.eth"; // Replace with the name you're querying
    let address = fetch_everyname_address(name).await?;
    println!("Address: {:?}", address);

    let address = "0xD475C94562cabC52e38cb3012D976185813284bb"; // Replace with the address you're querying
    let name = fetch_everyname_name(address).await?;
    println!("Name: {:?}", name);

    Ok(())
}



async fn fetch_everyname_address(name: &str) -> Result<Option<String>, reqwest::Error> {
    if let Some(api_key) = std::env::var("EVERYNAME_API_KEY").ok() {
        let url = format!(
            "https://api.everyname.xyz/forward?domain={}",
            reqwest::to_url_encoded(name)
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header("Accept", "application/json")
            .header("api-key", api_key)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let address = data["address"].as_str().map(|s| s.to_string());

        Ok(address)
    } else {
        Ok(None)
    }
}

async fn fetch_everyname_name(address: &str) -> Result<Option<String>, reqwest::Error> {
    if let Some(api_key) = std::env::var("EVERYNAME_API_KEY").ok() {
        let url = format!(
            "https://api.everyname.xyz/reverse?address={}&network=eth",
            reqwest::to_url_encoded(address)
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header("Accept", "application/json")
            .header("api-key", api_key)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let name = data["domain"].as_str().map(|s| s.to_string());

        Ok(name)
    } else {
        Ok(None)
    }
}
