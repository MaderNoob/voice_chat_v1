use cpal::{traits::DeviceTrait, Device, Stream, StreamConfig};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
pub struct AudioStreams {
    input: Stream,
    output: Stream,
}
pub fn build_streams<Cin: AsRef<StreamConfig>, Cout: AsRef<StreamConfig>>(
    input_device: &Device,
    input_config: Cin,
    input_sender: Sender<Vec<f32>>,
    output_device: &Device,
    output_condig: Cout,
    output_buffer: Arc<Mutex<Vec<f32>>>,
) -> std::io::Result<AudioStreams> {
    let input =
        input_device.build_input_stream(input_config.as_ref(), move |data, info| {
            input_sender.
        }, |e| {});
}
