use cpal::{traits::DeviceTrait, Device, Stream, StreamConfig};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
