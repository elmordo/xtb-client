use tracing::Level;
use xtb_client::{StreamApi};
use xtb_client::schema::{StreamGetKeepAliveSubscribe};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap_or_default();
    let subscriber = tracing_subscriber::fmt().with_max_level(Level::DEBUG).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let username = dotenvy::var("XTB_USERNAME").unwrap();
    let password = dotenvy::var("XTB_PASSWORD").unwrap();
    let api_server = dotenvy::var("XTB_API_SERVER").unwrap();
    let stream_server = dotenvy::var("XTB_STREAM_SERVER").unwrap();

    let mut client = xtb_client::XtbClientBuilder::new(&api_server, &stream_server).build(&username, &password).await.unwrap();

    let mut listener = client.subscribe_keep_alive(StreamGetKeepAliveSubscribe::default()).await.unwrap();

    while let Some(item) = listener.next().await.unwrap() {
        println!("Keep alive received: {}", item.timestamp);
    }

    println!("Stream closed");
}
