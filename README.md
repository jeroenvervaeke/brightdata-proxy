# brightdata-proxy
## Description
A simple helper library which helps you set up a brightdata proxy which you can use with the reqwest library.

## Example
A fully working example can be found [here](https://github.com/jeroenvervaeke/brightdata-proxy/blob/master/examples/simple/src/main.rs).

```rust
// Create a proxy builder
let proxy_builder = BrightdataProxyBuilder::builder()
    .username(username)
    .password(password)
    .build();

// Create a new proxy
// 1 session = 1 ip, as long as you use the same client you'll keep the same IP
// NOTE: you can only keep the same ip for 60 seconds, see docs: https://docs.brightdata.com/api-reference/proxy/rotate_ips
let proxy = proxy_builder
    .create_new_session()
    .context("create new proxy session")?;

// Build your request client and use as usual
let client = ClientBuilder::new()
    .proxy(proxy)
    .build()
    .context("build http client")?;
```
