use cpal::{
    host_from_id,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};
use std::io::Read;
use std::net::UdpSocket;
use std::{
    io::stdin,
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread::spawn,
};
mod control;

fn main() {
    spawn(control_thread);
    let _ = stdin().read(&mut [0u8; 1]).unwrap();
    ()
}

fn control_thread() {
    
}

fn main_channels() {
    let host = cpal::default_host();

    let (sender, receiver) = std::sync::mpsc::channel::<Vec<f32>>();
    let mut receiver_reader = ChannelBufRead::new(receiver);

    let total_send_time: Arc<Mutex<f64>> = Arc::new(Mutex::new(0f64));
    let total_send_time_clone = Arc::clone(&total_send_time);
    let total_read_time: Arc<Mutex<f64>> = Arc::new(Mutex::new(0f64));
    let total_read_time_clone = Arc::clone(&total_read_time);

    let indevice = host.default_input_device().unwrap();
    let inconfig = indevice.default_input_config().unwrap();
    let instream = indevice
        .build_input_stream(
            &inconfig.into(),
            move |data: &[f32], info| {
                let timer = std::time::Instant::now();
                sender.send(Vec::from(data));
                let send_time_clone = Arc::clone(&total_send_time);
                let mut send_time = send_time_clone.lock().unwrap();
                *send_time += timer.elapsed().as_secs_f64();
            },
            |e| {
                println!("shit: {}", e);
            },
        )
        .unwrap();

    let outdevice = host.default_output_device().unwrap();
    let outconfig = outdevice.default_output_config().unwrap();
    let outstream = outdevice
        .build_output_stream(
            &outconfig.into(),
            move |data: &mut [f32], info| {
                let timer = std::time::Instant::now();
                receiver_reader.read_into(data);
                let read_time_clone = Arc::clone(&total_read_time);
                let mut read_time = read_time_clone.lock().unwrap();
                *read_time += timer.elapsed().as_secs_f64();
            },
            |e| {
                println!("SHIT: {}", e);
            },
        )
        .unwrap();

    outstream.play().unwrap();
    instream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    outstream.pause().unwrap();
    instream.pause().unwrap();
    println!("Total send time: {}", total_send_time_clone.lock().unwrap());
    println!("Total read time: {}", total_read_time_clone.lock().unwrap());
}
fn main_mutex() {
    let host = cpal::default_host();

    let shared_buf = Arc::new(Mutex::new(Vec::new()));

    let total_send_time: Arc<Mutex<f64>> = Arc::new(Mutex::new(0f64));
    let total_send_time_clone = Arc::clone(&total_send_time);
    let total_read_time: Arc<Mutex<f64>> = Arc::new(Mutex::new(0f64));
    let total_read_time_clone = Arc::clone(&total_read_time);

    let indevice = host.default_input_device().unwrap();
    let inconfig = indevice.default_input_config().unwrap();

    let instream_buf = Arc::clone(&shared_buf);
    let send_time_clone = Arc::clone(&total_send_time);

    let instream = indevice
        .build_input_stream(
            &inconfig.into(),
            move |data: &[f32], info| {
                let timer = std::time::Instant::now();
                {
                    let mut buf = instream_buf.lock().unwrap();
                    buf.extend_from_slice(data);
                }
                let mut send_time = send_time_clone.lock().unwrap();
                *send_time += timer.elapsed().as_secs_f64();
            },
            |e| {
                println!("shit: {}", e);
            },
        )
        .unwrap();

    let outdevice = host.default_output_device().unwrap();
    let outconfig = outdevice.default_output_config().unwrap();

    let read_time_clone = Arc::clone(&total_read_time);
    let outstream_buf = Arc::clone(&shared_buf);

    let outstream = outdevice
        .build_output_stream(
            &outconfig.into(),
            move |data: &mut [f32], info| {
                let timer = std::time::Instant::now();
                {
                    let mut buf = outstream_buf.lock().unwrap();
                    if buf.len() >= data.len() {
                        data.copy_from_slice(&buf[..data.len()]);
                        buf.drain(0..data.len());
                        // for (index,sample) in buf.drain(0..data.len()).enumerate(){
                        //     data[index]=sample;
                        // }
                    }
                }
                let mut read_time = read_time_clone.lock().unwrap();
                *read_time += timer.elapsed().as_secs_f64();
            },
            |e| {
                println!("SHIT: {}", e);
            },
        )
        .unwrap();

    outstream.play().unwrap();
    instream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    outstream.pause().unwrap();
    instream.pause().unwrap();
    println!("Total send time: {}", total_send_time_clone.lock().unwrap());
    println!("Total read time: {}", total_read_time_clone.lock().unwrap());
}
pub struct ChannelBufRead {
    receiver: std::sync::mpsc::Receiver<Vec<f32>>,
    buf: Vec<f32>,
}
impl ChannelBufRead {
    pub fn new(receiver: std::sync::mpsc::Receiver<Vec<f32>>) -> ChannelBufRead {
        ChannelBufRead {
            receiver,
            buf: Vec::new(),
        }
    }
    pub fn read_into(&mut self, buf: &mut [f32]) {
        while self.buf.len() < buf.len() {
            self.buf.extend_from_slice(&self.receiver.recv().unwrap());
        }
        buf.copy_from_slice(&self.buf[0..buf.len()]);
        self.buf.drain(..buf.len());
        // for (index, value) in self.buf.drain(..buf.len()).enumerate() {
        //     buf[index] = value;
        // }
    }
}
