use std::cell::Cell;

use crossbeam_channel::{unbounded, Receiver, Sender};

/// Output channel used by the network handler.
pub type OutputReceiver<O> = Receiver<O>;

pub struct OutputSender<O> {
    output: Sender<O>,
    output_enabled: Cell<bool>,
}

impl<O> OutputSender<O> {
    pub fn new() -> (Self, OutputReceiver<O>) {
        let (sender, receiver) = unbounded();
        (
            Self {
                output: sender,
                output_enabled: Cell::new(true),
            },
            receiver,
        )
    }

    fn disable(&self) {
        info!("Disabling output");
        self.output_enabled.replace(false);
    }

    pub fn send(&self, output: O) {
        if !self.output_enabled.get() {
            return;
        };

        if self.output.send(output).is_err() {
            self.disable();
            error!("Output channel is disconnected");
        };
    }
}
