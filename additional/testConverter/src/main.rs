use std::{env, fs::File, path::Path, io::{Write}, collections::HashSet};

use calamine::{Reader, RangeDeserializerBuilder, open_workbook_auto};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct RowModel {
    id: String,
    number: String,
    width: i32,
    depth: i32,
    height: i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Must provide a use-case xls/xlsx file!");
    }

    let mut test_map = File::create("map.txt")?;
    let mut instance_id = 0;
    for file in args[1..].iter() {
        let mut excel = open_workbook_auto(file)?;
        let range = excel
        .worksheet_range_at(0)
        .ok_or(calamine::Error::Msg("Cannot find default sheet"))??;

        let iter_result =
            RangeDeserializerBuilder::new()
            .has_headers(false)
            .from_range::<_, RowModel>(&range)?;
        let out_path = Path::new(file).with_file_name(format!("instance-{}", instance_id)).with_extension("test");
        let mut out_file = File::create(out_path)?;

        writeln!(&mut test_map, "{:?} => {}", Path::new(file).file_name().unwrap(), format!("instance-{}.test", instance_id))?;
        instance_id += 1;

        let mut is_pallet = true;
        let mut i = 0;
        let mut item_types = HashSet::<String>::new();
        for raw_row in iter_result {
            if let Ok(row) = raw_row {
                if is_pallet {
                    writeln!(&mut out_file, "bin {},{},{}", row.width, row.depth, row.height)?;
                }
                else {
                    if !item_types.contains(&row.id) {
                        item_types.insert(row.id);
                        let number_values: Vec<&str> = row.number.split('-').collect();
                        let number = number_values.get(0).unwrap();
                        let number = str::parse::<i32>(number)?;
                        for _ in 0..number {
                            writeln!(&mut out_file, "box {},{},{},{}", i, row.width, row.depth, row.height)?;
                            i += 1;
                        }
                    }
                }
                is_pallet = false;
            }
        }
    }
    Ok(())
}
