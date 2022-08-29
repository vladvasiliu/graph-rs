use anyhow::Result;
use futures_util::StreamExt;
use graph_rs_sdk::client::Graph;
use graph_rs_sdk::oauth::OAuth;
use std::env;

// This demonstrates usage of the pagination interface
// It returns a stream that follows the `next_link` of each response.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut oauth = OAuth::new()
        .client_id(&env::var("CLIENT_ID").unwrap())
        .client_secret(&env::var("CLIENT_SECRET").unwrap())
        .tenant_id(&env::var("TENANT_ID").unwrap())
        .add_scope("https://graph.microsoft.com/.default")
        .build_async()
        .client_credentials();
    let token = oauth.access_token().send().await?;
    let graph_client = Graph::new_async(token.bearer_token());
    let request = graph_client.v1().applications();
    let mut response = request
        .list_application()
        .select(&["appId,displayName"])
        .top("2") // Make it small to demonstrate paging
        .paginate();

    let mut stream = response.enumerate();

    while let Some((count, value)) = stream.next().await {
        println!("\n\n=========== PAGE {} ===========", count);
        println!("{:#?}", value);
    }

    Ok(())
}
