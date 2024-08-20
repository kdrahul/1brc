// use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::ops::Add;

#[derive(Debug)]
struct Data {
    min: f32,
    max: f32,
    sum: f32,
    count: u32,
}

impl Data {
    fn update(&mut self, temp: f32) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum = self.sum.add(temp);
        self.count += 1;
    }
}

fn process_data(s: &str) -> (String, f32) {
    let mut splits = s.split(";");
    (
        splits.next().expect("error").to_string(),
        splits.next().expect("error").parse::<f32>().expect("error")
    )
}

fn data_intake(input: &mut BufReader<File>) -> HashMap<String, Data> {
    let mut buf = String::new();
    let mut map:HashMap<String, Data> = HashMap::new();

    while let Ok(bytes) = input.read_line(&mut buf) {
        if bytes == 0 {
            break;
        }
        let line = buf.trim();
        let (name, temp) = process_data(line);
        map.entry(name)
            .and_modify(|f| f.update(temp))
            .or_insert(Data {
                min: temp,
                max: temp,
                sum: temp,
                count: 1,
            });
        buf.clear();
    }
    map
}

fn main() {
    // Open the file
    let file = File::open("measurements.txt").unwrap();
    // Read all lines of the file.
    let mut file = io::BufReader::new(file);

    let mut map: HashMap<String, Data> = data_intake(&mut file);
    let mut writer = BufWriter::new(std::io::stdout());
    let _ = writer.write_all(b"{");

    let len = map.len();
    let mut count: usize = 0;

    map.iter().for_each(|(key, value)| {
        let min: f32 = value.min;
        let max: f32 = value.max;
        let avg: f32 = value.sum / value.count as f32;

        count += 1;
        let s = &format!("{}={:.1}/{:.1}/{:.1}", &key, min, avg, max);
        writer.write_all(s.as_bytes()).unwrap();
        if len > count {
            writer.write_all(b", ").unwrap();
        }
    });

    writer.write_all(b"}").unwrap();
    writer.flush().unwrap();
}
