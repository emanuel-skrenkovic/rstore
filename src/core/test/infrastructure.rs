use ::eventstore::{All, Client, ReadResult, ReadStreamOptions, StreamPosition};
use rocket::futures::StreamExt;
use serde::de::DeserializeOwned;

pub struct EventStoreFixture {
    client: Client,
}

impl EventStoreFixture {
    pub async fn create() -> EventStoreFixture {
        let connection_string = "esdb://localhost:2113?tls=false&tlsVerifyCert=false";
        let settings = connection_string
            .parse()
            .expect("Failed to parse the EventStore connection string.");

        EventStoreFixture {
            client: Client::create(settings)
                .await
                .expect("Failed to create EventStore client."),
        }
    }

    pub async fn events<TEvent: DeserializeOwned>(&self, stream_name: &str) -> Vec<TEvent> {
        let options = ReadStreamOptions::default()
            .position(StreamPosition::Start)
            .forwards();

        let event_stream = self
            .client
            .read_stream(&stream_name, &options, All)
            .await
            .unwrap();

        if let ReadResult::Ok(mut stream) = event_stream {
            let mut events: Vec<TEvent> = vec![];

            while let Some(event) = stream.next().await {
                let correct_event = event.unwrap();

                let domain_event = correct_event
                    .get_original_event()
                    .as_json::<TEvent>()
                    .unwrap();
                events.push(domain_event);
            }

            return events;
        }

        return vec![];
    }
}

// TODO: think about having a DockerContainer trait

/*
use docker::Docker;

pub struct DockerFixture {
    client: Docker
}

impl DockerFixture {
    pub fn new() -> DockerFixture {
        // TODO: Windows
        let client = Docker::connect("unix:///var/run/docker.sock")
            .expect("Could not connect to Docker.");

        DockerFixture {
            client
        }
    }

    pub async fn start() {
        todo!()
    }

    pub async fn stop() {
        todo!()
    }
}
 */
