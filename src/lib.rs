extern crate num;
#[macro_use]
extern crate num_derive;
pub mod message;
pub mod doip_server;
pub mod doip_client;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
