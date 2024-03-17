use xtb_client::ApiClient;
use xtb_client::schema::GetAllSymbolsRequest;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap_or_default();
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let username = dotenvy::var("XTB_USERNAME").unwrap();
    let password = dotenvy::var("XTB_PASSWORD").unwrap();
    let api_server = dotenvy::var("XTB_API_SERVER").unwrap();
    let stream_server = dotenvy::var("XTB_STREAM_SERVER").unwrap();

    let mut client = xtb_client::XtbClientBuilder::new(&api_server, &stream_server).build(&username, &password).await.unwrap();

    let symbols = client.get_all_symbols(GetAllSymbolsRequest::default()).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&symbols).unwrap())
}
