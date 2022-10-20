use image::{self, ImageBuffer, Luma, LumaA};

use crate::map::RoadId;

use super::map::LaneId;
use std::collections::HashMap;
use std::fs::{self, DirEntry, FileType};
use std::path::Path;

pub fn load_road_masks() -> HashMap<LaneId, ImageBuffer<LumaA<u8>, Vec<u8>>> {
    let mut masks = HashMap::new();

    for entry in fs::read_dir("../road-masks").expect("Failed to open road mask directory") {
        if let Err(_) = entry {
            continue;
        }
        let entry = entry.unwrap();
        let filetype = entry.file_type();
        if let Err(_) = filetype {
            continue;
        }
        if !filetype.unwrap().is_file() {
            continue;
        }

        let name = entry.file_name().into_string();
        if let Err(e) = name {
            continue;
        }
        let name = name.unwrap();
        let parts = name.split('.').collect::<Vec<&str>>();
        if parts.len() != 2 {
            continue;
        }
        let parts = parts.get(0).unwrap().split('-').collect::<Vec<&str>>();
        if parts.len() != 3 {
            continue;
        }
        let int1id = if let Ok(v) = parts.get(0).unwrap().parse::<u32>() {
            v
        } else {
            continue;
        };
        let int2id = if let Ok(v) = parts.get(1).unwrap().parse::<u32>() {
            v
        } else {
            continue;
        };

        println!("{:?}", entry.path());
        let lane_id = if *parts.get(2).unwrap() == "l" { 0 } else { 1 };
        let image = image::open(entry.path()).unwrap();
        let image = image.grayscale();
        let image = image.as_luma_alpha8().unwrap();
        masks.insert(LaneId(RoadId(int1id, int2id), lane_id), image.clone());
    }

    masks
}
