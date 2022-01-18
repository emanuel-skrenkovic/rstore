use crate::core::result::{Error, Result};

use crate::core::aggregate::AggregateEntity;
use eventstore::{
    All, Client, EventData, ReadResult, ReadStreamOptions, StreamPosition, WrongExpectedVersion,
};
use rocket::futures::StreamExt;
use rocket::serde::Serialize;
use serde::de::DeserializeOwned;

pub struct AggregateStore {
    client: Client,
}

impl AggregateStore {
    pub async fn create() -> AggregateStore {
        // TODO: parameterize?
        let connection_string = "esdb://localhost:2113?tls=false&tlsVerifyCert=false";
        let settings = connection_string
            .parse()
            .expect("Failed to parse the EventStore connection string.");

        // TODO: refactor to single connection per application! (I think).
        AggregateStore {
            client: Client::create(settings)
                .await
                .expect("Failed to create EventStore client."),
        }
    }

    pub async fn load<
        TEntity: AggregateEntity<TEventBase> + Default,
        TEventBase: DeserializeOwned,
    >(
        &self,
        id: String,
    ) -> Result<TEntity> {
        let options = ReadStreamOptions::default()
            .position(StreamPosition::Start)
            .forwards();

        let event_stream = self.client.read_stream(&id, &options, All).await?;

        if let ReadResult::Ok(mut stream) = event_stream {
            let mut events: Vec<TEventBase> = vec![];

            while let Some(event) = stream.next().await {
                let correct_event = event.unwrap();

                let domain_event = correct_event
                    .get_original_event()
                    .as_json::<TEventBase>()
                    .unwrap();
                events.push(domain_event);
            }

            let agg: TEntity = TEntity::hydrate(events);
            return Result::Ok(agg);
        } else if let ReadResult::StreamNotFound(not_found_error) = event_stream {
            return Result::Err(Box::new(Error::NotFound {
                message: not_found_error,
            }));
        }

        return Result::Err(Box::new(Error::Internal {
            message: format!(
                "Unexpected error occurred while reading from stream '{}'.",
                &id
            ),
        }));
    }

    pub async fn save<TEntity: AggregateEntity<TEventBase>, TEventBase: Serialize>(
        &self,
        stream_name: &str,
        agg: TEntity,
    ) -> Result<()> {
        let payload: Vec<EventData> = agg
            .uncommitted_events()
            .iter()
            .map(|event| EventData::json(std::any::type_name::<TEventBase>(), event).unwrap())
            .collect();

        let write_result = self
            .client
            .append_to_stream(stream_name, &Default::default(), payload)
            .await?;

        if let Err(WrongExpectedVersion { current, expected }) = write_result {
            Result::Err(Box::new(Error::Internal {
                message: format!(
                    "Wrong expected version. Expected: {:?}, current: {:?}",
                    expected, current
                ),
            }))
        } else {
            Result::Ok(())
        }
    }
}
