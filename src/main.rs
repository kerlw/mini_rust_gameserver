extern crate tokio;
extern crate websocket;

use websocket::futures::future;
use websocket::r#async::Server;

fn main() {
    let server = Server::bind("0.0.0.0:3344", &tokio::reactor::Handle::default()).unwrap();

    server.incoming().then(future::ok);
}
