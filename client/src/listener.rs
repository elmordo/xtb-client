use async_trait::async_trait;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::task::JoinHandle;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{error};
use crate::message_processing;
use crate::message_processing::ProcessedMessage;


/// Helper type making variable and field declaration shorter.
pub type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;


/// Handler trait used to avoid using async callbacks
#[async_trait]
pub trait ResponseHandler: Send + Sync + 'static {
    /// Process given response.
    ///
    /// The logic must be "safe" - it should not panic
    async fn handle_response(&self, response: ProcessedMessage);
}


/// Spawn listener for command responses. Responses are handled by `response_handler`
pub fn listen_for_responses(mut stream: SplitStream<Stream>, response_handler: impl ResponseHandler) -> JoinHandle<()> {
    spawn(async move {
        let response_handler = response_handler;
        // Read messages until some is delivered
        while let Some(message_result) = stream.next().await {
            let message = match message_result {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Error when receiving message: {:?}", err);
                    continue;
                }
            };
            // process message
            let response = match message_processing::process_message(message) {
                Ok(response) => response,
                Err(err) => {
                    error!("Cannot process response: {:?}", err);
                    continue
                },
            };
            response_handler.handle_response(response).await;
        }
    })
}
