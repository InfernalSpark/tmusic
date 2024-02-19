use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub fn play() {
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open("src/sample_audio/nigga_money.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}