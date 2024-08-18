use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Data {
    city: String,
    temp: f32,
}

fn process_data(s: &str) -> Data {
    let splits: Vec<&str> = s.split(";").collect();
    Data {
        city: splits.get(0).unwrap().to_string(),
        temp: splits.get(1).unwrap().parse::<f32>().unwrap(),
    }
}

fn find_values(v: &Vec<f32>) -> [f32; 3] {
    let mut min: f32 = 0.0;
    let mut sum: f32 = 0.0;
    let mut max: f32 = 0.0;

    for i in 0..v.len() {
        min = min.min(v[i]);
        max = max.max(v[i]);
        sum += v[i];
    }
    [min, sum / v.len() as f32, max]
}

fn main() {
    let file = File::open("measurements.txt").unwrap();
    // Read all lines of the file
    let mut iter = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap());
    let mut map: HashMap<String, Vec<f32>> = HashMap::new();


    while let Some(t) = iter.next() {
        let d = process_data(&t);
        map.entry(d.city)
            .and_modify(|f| f.push(d.temp))
            .or_insert(vec![d.temp]);
    }

    let mut final_string: String = String::new();
    final_string.push('{');

    let all_strs :Vec<String> = map.iter().map(|(key,value)|{
        let min: f32;
        let max: f32;
        let avg: f32;

        let res: [f32; 3] = find_values(value);

        min = res[0];
        avg = res[1];
        max = res[2];
        format!("{}={:.1}/{:.1}/{:.1}", &key, min, avg, max)
    }).collect();

    final_string.push_str(&all_strs.join(", "));
    final_string = final_string[0..final_string.len() - 2].to_string();
    final_string.push('}');
    println!("{}", final_string);
}
