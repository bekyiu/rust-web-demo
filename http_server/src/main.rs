mod server;
mod router;
mod handler;

fn main() {
    let server = server::Server::new("127.0.0.1:9000");
    server.run();
}
