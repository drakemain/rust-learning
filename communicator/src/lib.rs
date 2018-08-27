pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
  use super::{client, network, network::server};

    #[test]
    fn it_works() {
        client::connect();
        network::connect();
        server::connect();
    }
}
