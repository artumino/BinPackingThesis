use std::{env, fs::File, path::Path, io::{Write}, collections::HashSet};

use calamine::{Reader, RangeDeserializerBuilder, open_workbook_auto};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct LiteratureRowModel {
    class_instance: i32,
    n: i32,
    execution_time: f32,
    opened_bins: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct OurRowModel {
    class_instance: i32,
    n: i32,
    k: i32,
    execution_time: f32,
    opened_bins: f32,
    cage_ratio: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Must provide a csv test file and a literature folder!");
    }

    let test_file = Path::new(&args[1]);
    let literature_dir = Path::new(&args[2]);
    let out_table = Path::new(&args[1]).with_extension("tex");
    

    let mut excel = open_workbook_auto(test_file)?;
    let range = excel
    .worksheet_range_at(0)
    .ok_or(calamine::Error::Msg("Cannot find default sheet"))??;

    let iter_result =
        RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range::<_, OurRowModel>(&range)?;

    let mut out_file = File::create(out_table)?;

    for raw_row in iter_result {
        if let Ok(row) = raw_row {
        }
    }
    Ok(())
}
