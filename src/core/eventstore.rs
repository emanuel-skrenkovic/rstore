use crate::error::result::Result;

use eventstore::Client;

// TODO: TODO
pub async fn client() -> Result<Client> {
    let connection_string = "esdb://localhost:2113?tls=false&tlsVerifyCert=false";
    let settings = connection_string.parse()?;

    Client::create(settings).await
}
