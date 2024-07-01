use crossbeam_channel::{unbounded, Receiver, Sender};

/// Output channel used by the network handler.
pub type OutputReceiver<O> = Receiver<O>;

pub struct OutputSender<O> {
    output: Sender<O>,
    output_enabled: bool,
}

impl<O> OutputSender<O> {
    pub fn new() -> (Self, OutputReceiver<O>) {
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
        info!("Disabling output");
        self.output_enabled = false;
    }

    pub(crate) fn send(&mut self, output: O) {
        if !self.output_enabled {
            return;
        };

        if self.output.send(output).is_err() {
            self.disable();
            error!("Output channel is disconnected");
        };
    }
}
