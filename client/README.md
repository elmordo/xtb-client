# XTB-Client

The XTB-Client library provides typed and simple to use implementation of a connector to the 
[API](developers.xstore.pro/documentation/) of the [XTB](https://www.xtb.com/) broker.

* programming style: **async**
* framework: **tokio**
* XTB API version: **2.5**
* Request/response API: **supported** 
* Stream API: **supported** 

## Warning

This library is still under development. The library api can be changed and some bugs are maybe living in the code. 

## How to use

The XTB interface is provided by the `XtbClient` struct implementing the `XtbApi` and `XtbStreamApi` traits. The struct
can be created by the `XtbClientBuilder` struct and when created, it is automatically connected to the server and
the `login` command is performed. If both is success, the `XtbClient` instance is returned.

### Configuration

The `XtbClient` is configured by the `XtbClientBuilder`. The builder has three initialization methods:

* `new(api_url: &str, stream_api_url: &str)` - create new instance with custom urls.
* `new_real()` - create new instance with real account servers preset.
* `new_demo()` - create new instance with demo account servers preset.
* `new_bare()` - create new instance without any value set.

The setters (prefixed by `with_` and options are flattened to underlying type) and getters (not prefixed) can be used 
to override following attributes:

* `api_url: Option<String>` - URL of the request/response api server.
* `stream_api_url: Option<String>` - URL of the stream api server.
* `app_id: Option<String>` - optional application id (marked as deprecated in the official documentation).
* `app_name: Option<String>` - optional application name (marked as deprecated in the official documentation).
* `ping_period: Option<u64>` - time period between ping requests (default is 30s)

When a builder instance is configured, the `build` method can be called. The method accepts two `&str` params: username and password.

When the method is called, server connections are created, user is logged in and ping tokio green threads are spawned.

```rust
use xtb_client::XtbClientBuilder;


#[tokio::main]
fn main() {
  let real_builder = XtbClientBuilder::new_real();
  let demo_builder = XtbClientBuilder::new_demo();
  let custom_builder = XtbClientBuilder::new("wss://my-server.com/api", "wss://my-server.com/apiStream").with_ping_period(20u64);
  let bare_builder = XtbClientBuilder::new_bare().with_api_url("wss://my-server.com/api").with_stream_api_url("wss://my-server.com/apiStream");
  
  let client = custom_builder.build("12345", "reallySecretPassword");
}

```

### Request/response API

The simplest way how to communicate with the server is a request/response API. This api always returns a response to
a request.

Methods names corresponds with the command names transformed to snake_case. Examples:

* `getAllSymbols` -> `get_all_symbols`
* `getCalendar` -> `get_calendar`
* `getVersion` -> `get_version`

Several commands are not implemented in public interface of the trait:

* `login` - performed automatically by `XtbClient` creation process
* `logout` - performed automatically when `XtbClient` instance is dropped
* `ping` - performed periodically by the `XtbClient` instance every 30s (can be configured).

### Stream API

The stream api feed a consumer by stream of messages delivered to a consumer by the `DataStream` struct. The subscription 
is cancelled automatically when a `DataStream` instance is dropped.

The stream API implementation uses internal subscription counter so if two subscribers are subscribed for same messages
(e.g. USDEUR tick prices), unsubscription of the first one does not cut off the second one from the messages.

Methods names corresponds with the command names transformed to snake_case and with `subscribe_` prefix instead 
of the `get_` prefix. Examples:

* `getCandles` -> `subscribe_candles`
* `getTrades` -> `subscribe_trades`
* `getProfits` -> `subscribe_profits`

Several commands are not implemented in public interface of the trait:

* `ping` - performed periodically by the `XtbClient` instance every 30s (can be configured).

## Low level interface

The library exposes low level connections too.

### Request/response connection

The `XtbConnection` trait and its implementor `BasicXtbConnection` provides low level connection to the XTB server
for the request-response commands.

The trait provides the `send_command` method. This method accepts command name (for example `getNews`) and payload
used as a command arguments. When the command has no arguments, pass `None`.

After the method is called, the `ResponsePromise` struct instance is returned. It is awaitable returning response from
the server. The result is `Ok(Response)` if command was successful or `Err(ErrorResponse)` if command fails.

### Stream connection

The `XtbStreamConnection` trait and its implementor `BasicStreamConnection` provides low level connection to the XTB
stream server for data streaming.

The trait provides `subscribe`, `unsubscribe` and `make_message_stream` methods. 

The `subscribe` method accepts command name and optional arguments as parameter. The parameter must be `None` 
or `Some(serde_json::Value::Null)` for commands without arguments and `Some(serde_json::Value::Object)` for commands 
with arguments. When command is send successfully, the server should stream requested data.

The `unsubscribe` method is similar to the `subscribe` but its effect is to stop the data stream.

The `make_message_stream` make local subscription for messages delivered from the XTB server. It accepts 
`DataMessageFilter` enum as parameter and use it to filter incoming messages by given predicate. Available filters are
following:

* `Always` (default) - all messages are passed to a consumer
* `Never` - no message is passed for consumer
* `Command(String)` - filter messages by the `command` field of data message received from the server
* `FieldValue { name: String, value: serde_json::Value }` - filter messages by field name of `data` field of the data message 
received from the server. The filter match if and only if `data` contains type of `serde_json::Value::Object` and
the object has key named `name` and the field contains equal value as `value`.
* `Custom(Box<dyn Fn(&StreamDataMessage) -> bool + Send + Sync>)` - custom filter fn
* `All(Vec<DataMessageFilter>)` - container for none, one or more predicates. Matches if and only if all predicates is
matching. If list of predicates is empty, returns `true`.
* `Any(Vec<DataMessageFilter>)` - container for none, one or more predicates. Matches if any predicate is
  matching. If list of predicates is empty, returns `false`.

Note: the `Not` is not implemented but the `Custom` variant can be used to create it.

The `make_message_stream` returns implementor of the `MessageStream` trait. This trait provides the `next()` method
returning incoming messages matching to the filter.

## Examples

### Example 1

Simple API call to get all available symbols.

```rust
use tracing::Level;
use xtb_client::CommandApi;
use xtb_client::schema::GetAllSymbolsRequest;

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

    let symbols = client.get_all_symbols(GetAllSymbolsRequest::default()).await.unwrap();
    println!("{}", serde_json::to_string_pretty(&symbols).unwrap())
}

```

### Example 2

Subscribe for the `keppAlive` messages.

```rust
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

```

# Buy me a ~~coffee~~ beer

If you like this library, support me by one cold [beer](https://www.buymeacoffee.com/elmordo). The beer is tasty and full of vitamins :-)
