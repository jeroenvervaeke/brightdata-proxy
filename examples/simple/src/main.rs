use std::env;

use anyhow::{Context, Result};
use brightdata_proxy::BrightdataProxyBuilder;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("PROXY_USERNAME").context("reading PROXY_USERNAME")?;
    let password = env::var("PROXY_PASSWORD").context("reading PROXY_PASSWORD")?;

    let proxy_builder = BrightdataProxyBuilder::builder()
        .username(username)
        .password(password)
        .build();

    let first_ip = get_ip(&proxy_builder).await.context("get first ip")?;
    let second_ip = get_ip(&proxy_builder).await.context("get second ip")?;

    println!("Fetched 2 ips:");
    println!("- {first_ip}");
    println!("- {second_ip}");

    Ok(())
}

async fn get_ip(proxy_builder: &BrightdataProxyBuilder) -> Result<String> {
    let proxy = proxy_builder
        .create_new_session()
        .context("create new proxy session")?;

    let client = ClientBuilder::new()
        .proxy(proxy)
        .build()
        .context("build http client")?;

    let ip = client
        .get("https://api.ipify.org/?format=text")
        .send()
        .await
        .context("http request")?
        .error_for_status()
        .context("verify status code")?
        .text()
        .await
        .context("get body text")?;

    Ok(ip)
}
