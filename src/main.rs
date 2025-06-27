use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

static FILENAME: &str = "./data/weather_stations.csv";

fn parse_file(filename: &str) -> BTreeMap<String, Vec<f32>> {
    let mut stations: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            if line.is_empty() { continue; }
            if line.starts_with('#') { continue; }
            let record: Vec<_> = line.split(';').collect();
            let station_name = record[0].to_string();
            let temperature = record[1].parse::<f32>().unwrap();
            let mut temperatures = vec![];
            if stations.contains_key(&station_name) {
                temperatures = stations.get(&station_name).unwrap().to_vec();
            }
            temperatures.push(temperature);
            stations.insert(station_name, temperatures);
        }
    }
    return stations;
}

fn compute_mean(temperatures: Vec<f32>) -> f32 {
    let sum: f32 = temperatures.iter().sum();
    return sum / temperatures.len() as f32;
}

fn compute_min_max(temperatures: Vec<f32>) -> (f32, f32) {
    let mut min = temperatures[0];
    let mut max = temperatures[0];
    for temperature in temperatures {
        if temperature < min { min = temperature; }
        if temperature > max { max = temperature; }
    }
    return (min, max);
}

fn main() {
    let stations = parse_file(FILENAME);

    let mut result = String::from("{");
    for (station_name, temperature) in &stations {
        let mean = compute_mean(temperature.to_vec());
        let (min, max) = compute_min_max(temperature.to_vec());
        result += format!("{}={:?}/{:?}/{:?}, ", station_name, min.round(), mean.round(), max.round()).as_str();
    }
    result.pop();
    result.pop();
    result += "}";
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
