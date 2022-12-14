use std::env;
use std::fs;
use std::path::PathBuf;
use rayon::prelude::*;
use std::fs::write;
use std::time::Instant;
use log::{info, trace, warn, debug};
use env_logger;

fn bbox_xywh_to_segment(bbox: &[f64]) -> [f64; 8] {
    // center, width height
    if let [x, y, width, height] = bbox {
        let x1 = x - width / 2.0;
        let y1 = y - height / 2.0;
        let x2 = x1 + width;
        let y2 = y1 + height;
        return [x1, y1, x1, y2, x2, y2, x2, y1];
    } else { panic!("Boom")}
}


fn process_file(path: &PathBuf) -> (String, i32) {
    let mut num_bbox_fixed = 0;
    let mut should_update = false;
    let content = fs::read_to_string(path).unwrap();
    let mut new_lines: Vec<String> = Vec::new();
    for line in content.split('\n').map(|x| String::from(x)) {
        let split_line = line.split_whitespace();
        let count = split_line.clone().count();
        match count {
            5 => {
                let numbers: Vec<f64> = split_line.map(|s| s.parse().unwrap()).collect();
                // we do not copy data, we refer to the slice from the splitted line
                let segment = bbox_xywh_to_segment(&numbers[1..5]);
                let parsed_segment: String = segment.map(|x| x.to_string()).to_vec().join(" ");
                new_lines.push(parsed_segment);
                should_update = true;
                num_bbox_fixed += 1
            }
            _ => new_lines.push(line),
        }
    }
    if should_update {
        write(path, new_lines.join("\n")).expect(format!("[{:?}] Error writing.", path).as_str());
    }
    (
        path.file_name().unwrap().to_str().unwrap().to_string(),
        num_bbox_fixed,
    )
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).expect("You should pass a directory");
    info!("Reading files in {}", dir);
    let paths = fs::read_dir(dir).unwrap();
    let files: Vec<_> = paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.path())
        .collect();

    let num_files = files.len();
    let now = Instant::now();

    let stats: Vec<(String, i32)> = files
        .into_par_iter()
        .map(|file| process_file(&file))
        .collect();

    let elapsed = now.elapsed();

    for stat in stats {
        let fixed_bboxes = stat.1;
        if fixed_bboxes > 0 {
            debug!("{} - fixed {} bboxes.", stat.0, stat.1)
        };
    }
    info!(
        "Processed {} files in {} ms",
        num_files,
        elapsed.as_millis()
    );
}
