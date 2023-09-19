use arma_rs::{Context, ContextState};
use arma_rs::{IntoArma, Value};
use crossbeam_channel::Receiver;
use message_io::network::ResourceId;

use crate::CurrentHandler;

const CALLBACK_NAME: &str = "collab_eden";

#[derive(strum::IntoStaticStr)]
pub enum Callback {
    ServerStarted,
    ServerStopped,
    ServerClientConnected(ResourceId),
    ServerClientDisconnected(ResourceId),

    ClientConnected(bool),
    ClientDisconnected(bool),

    RemoteEvent(String, Value),
}

/// QOL macro that calls `to_arma` on each entry
macro_rules! value_vec {
    ($($x:expr),*) => {
        vec![$($x.to_arma()),*]
    };
}

impl Callback {
    #[must_use]
    /// Returns the variant name in camelCase, used as the event name in Arma
    fn event_name(&self) -> String {
        let variant_name: &'static str = self.into();

        // Convert first letter to lowercase
        let mut c = variant_name.chars();
        c.next().map_or_else(String::new, |f| {
            f.to_lowercase().collect::<String>() + c.as_str()
        })
    }

    #[must_use]
    pub fn data(self) -> (String, Vec<Value>) {
        (
            self.event_name(),
            match self {
                Self::ServerStarted | Self::ServerStopped => value_vec![],
                Self::ServerClientConnected(id) | Self::ServerClientDisconnected(id) => {
                    value_vec![id.to_string()]
                }
                Self::ClientConnected(succeeded) => {
                    value_vec![succeeded]
                }
                Self::ClientDisconnected(lost_connection) => value_vec![lost_connection],
                Self::RemoteEvent(event, data) => vec![event.to_arma(), data],
            },
        )
    }
}

pub fn start_callback_handler(ctx: Context, recv: Receiver<Callback>) {
    std::thread::spawn(move || {
        while let Ok(callback) = recv.recv() {
            if matches!(
                callback,
                Callback::ServerStopped
                    | Callback::ClientConnected(false)
                    | Callback::ClientDisconnected(_)
            ) {
                let current_handler = ctx
                    .global()
                    .get::<CurrentHandler>()
                    .expect("current handler is set on extension creation");
                current_handler.set(None);
            }

            let (func, data) = callback.data();
            debug!("callback: {func}({data:?})");
            ctx.callback_data(CALLBACK_NAME, &func, data).unwrap();
        }
    });
}
