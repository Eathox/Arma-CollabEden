use std::time::Instant;

use crossbeam_channel::{unbounded, Receiver, Sender};

pub mod client;
pub mod client_host;
pub mod server;

use crate::network::NetworkSerde;

pub use client::{ClientCommand, ClientHandler, ClientOutput};
pub use client_host::{ClientHostHandler, ClientHostOutput};
pub use server::{ServerCommand, ServerHandler, ServerOutput};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    #[serde(with = "instant_serde")]
    Ping(Instant),
    #[serde(with = "instant_serde")]
    Pong(Instant),

    ArmaEvent(String, arma_rs::Value),
}

impl NetworkSerde for Message {}

/// Serde impls for [`std::time::Instant`] to be used with `#[serde(with = "instant_serde")]`. Implemented by converting to and from [`std::time::Duration`].
mod instant_serde {
    use std::time::{Duration, Instant};

    use serde::{de::Error, Deserialize, Serialize};

    pub fn serialize<S: serde::Serializer>(
        instant: &Instant,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let duration = instant.elapsed();
        duration.serialize(serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Instant, D::Error> {
        let duration = Duration::deserialize(deserializer)?;
        Instant::now()
            .checked_sub(duration)
            .ok_or_else(|| Error::custom("instant is out of bounds"))
    }
}

pub type OutputReceiver<O> = Receiver<O>;

struct OutputSender<O> {
    output: Sender<O>,
    output_enabled: bool,
}

impl<O> OutputSender<O> {
    fn new() -> (Self, Receiver<O>) {
        let (sender, receiver) = unbounded();
        (
            Self {
                output: sender,
                output_enabled: true,
            },
            receiver,
        )
    }

    fn disable(&mut self) {
        self.output_enabled = false;
    }

    fn send(&mut self, output: O) {
        if !self.output_enabled {
            return;
        };

        if self.output.send(output).is_err() {
            self.disable();
            error!("Output channel is disconnected");
        };
    }
}
