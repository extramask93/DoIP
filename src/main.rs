

use doip_lib::doip_server::{DoIPServerBuilder, DoIPServer};

fn main() {
    let builder = DoIPServerBuilder::new();
    let server = builder.get_server();
    server.start();
    println!("dupa");
}
