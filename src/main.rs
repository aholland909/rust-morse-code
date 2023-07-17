use std::collections::HashMap;
use std::env;
use std::{thread, time};

fn encode_string(input: &String) -> String {
    let codes = HashMap::from([
        ("a", ".-"),
        ("b", "-..."),
        ("c", "-.-."),
        ("d", "-.."),
        ("e", "."),
        ("f", ".._."),
        ("g", "--."),
        ("h", "...."),
        ("i", ".."),
        ("j", ".---"),
        ("k", "-.-"),
        ("l", ".-.."),
        ("m", "--"),
        ("n", "-."),
        ("o", "---"),
        ("p", ".--."),
        ("q", "--.-"),
        ("r", ".-."),
        ("s", "..."),
        ("t", "-"),
        ("u", "..-"),
        ("v", "...-"),
        ("w", ".--"),
        ("x", "-..-"),
        ("y", "-.--"),
        ("z", "--.."),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        ("0", "-----"),
    ]);

    let mut output = String::from("");

    for (x, c) in input.chars().enumerate() {
        if c == ' ' {
            output.push('/');
        } else {
            output.push_str(codes.get(&c.to_string().to_lowercase() as &str).unwrap());
        }
        if x != input.len() - 1 {
            output.push(' ');
        }
    }
    return output;
}

fn generate_sound(input: &str) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.1);
    let period = time::Duration::from_millis(50);

    for i in input.chars() {
        if i == '.' {
            sink.append(rodio::source::SineWave::new(600.00));
            thread::sleep(period);
            sink.stop();
        } else if i == '-' {
            sink.append(rodio::source::SineWave::new(600.00));
            thread::sleep(period * 2);
            sink.stop();
        } else if i == ' ' {
            sink.append(rodio::source::SineWave::new(0.00));
            thread::sleep(period * 2);
            sink.stop();
        } else if i == '/' {
            sink.append(rodio::source::SineWave::new(0.00));
            thread::sleep(period * 5);
            sink.stop();
        }
        sink.append(rodio::source::SineWave::new(0.00));
        thread::sleep(period);
        sink.stop();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = encode_string(input);
    println!("Input: {}", input);
    println!("Output: {}", output);
    if &args.len() > &2 && &args[2] == "-s" {
        generate_sound(&output);
    }
}
