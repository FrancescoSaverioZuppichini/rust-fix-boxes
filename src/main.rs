use std::fs;
use std::env;
use std::path::PathBuf;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn bbox_xywh_to_segment(bbox: [f64;4]) -> [f64;8] {
    // center, width height
    let [x, y, width, height] = bbox;
    let x1 = x - width / 2.0;
    let y1 =  y - height / 2.0;
    let x2 = x1 + width;
    let y2 = y1 + height;
    [x1, y1, x1, y2, x2, y2,  x2, y1]
}


fn process_file(path: &PathBuf) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let mut new_lines: Vec<String> = Vec::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        let split_line = line.split_whitespace();
        let count = split_line.clone().count();
        match count {
            5 => {
                let numbers: Vec<f64> = split_line.map(|s| s.parse().unwrap()).collect();
                let mut bbox: [f64; 4] = [0.0;4];
                bbox.copy_from_slice(&numbers[1..5]);
                println!("{:?}", bbox_xywh_to_segment(bbox));
            },
            _ => new_lines.push(line)  
        }
        // new_lines.push(line.unwrap());
    }
}

fn main() {
    // println!("{}", sum_of_squares(&[2,3,4,5,6,7,8,9,10,12,13,14,15,16, 100, 200, 300]))
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).expect("You should pass a directory");
    println!("Reading files in {}", dir);
    let paths = fs::read_dir(dir).unwrap();
    let files: Vec<_> = paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.path())
        .collect();

    let mut count = 0;
    for file in files {
        process_file(&file);
        count += 1;
    }

    println!("Processed {} files.", count)

}
