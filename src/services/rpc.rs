use solana_client::rpc_client::RpcClient;
use std::time::Duration;

pub fn build_rpc_client() -> RpcClient {
    let primary = std::env::var("SOLANA_RPC_PRIMARY")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

    let fallback_urls: Vec<String> = std::env::var("SOLANA_RPC_FALLBACK")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    let urls = std::iter::once(primary)
        .chain(fallback_urls)
        .collect::<Vec<_>>();

    for url in urls {
        let client = RpcClient::new_with_timeout(url.clone(), Duration::from_secs(10));
        if client.get_health().is_ok() {
            tracing::info!("✅ Connected to Solana RPC: {}", url);
            return client;
        } else {
            tracing::warn!("⚠️ RPC {} not healthy, trying next...", url);
        }
    }

    panic!("❌ No healthy Solana RPC available");
}
