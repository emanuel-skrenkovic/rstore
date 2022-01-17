use eventstore::{Client};

// TODO: TODO
pub async fn client() -> Client {
    let settings = "esdb://localhost:2113?tls=false&tlsVerifyCert=false".parse(); // TODO
    Client::create(settings.unwrap()).await.unwrap()
}