
use reqwest;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    if let Some(api_key) = std::env::var("NEXT_PUBLIC_EVERYNAME_API_KEY").ok() {
        let name = "example"; // Replace with the name you're querying
        let address = fetch_everyname_address(&name, &api_key).await?;
        let name = fetch_everyname_name(&address, &api_key).await?;
        println!("Name: {:?}", name);
        println!("Address: {:?}", address);
    } else {
        println!("API key not set.");
    }

    Ok(())
}


async fn fetch_everyname_address(
    name: &str,
    api_key: &str,
) -> Result<Option<String>, reqwest::Error> {
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
}

async fn fetch_everyname_name(
    address: &str,
    api_key: &str,
) -> Result<Option<String>, reqwest::Error> {
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
}