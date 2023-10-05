use csv::Writer;
use serde_json::Value;
use std::error::Error;
use std::fs;
use structopt::StructOpt;

fn combinations_helper<T: Clone>(vectors: &Vec<Vec<T>>, index: usize) -> Vec<Vec<T>> {
    if index == vectors.len() {
        return vec![vec![]];
    }

    let rest = combinations_helper(vectors, index + 1);
    let current = &vectors[index];

    let mut result = Vec::new();
    for item in current {
        for comb in &rest {
            let mut combination = vec![item.clone()];
            combination.extend_from_slice(comb);
            result.push(combination);
        }
    }

    result
}

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(
        short = "o",
        long = "output",
        help = "Sets the output file name (.csv file)"
    )]
    output: String,

    #[structopt(
        short = "i",
        long = "input",
        help = "JSON files to process (.json file)"
    )]
    input: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let output_file = args.output;

    let mut result: Vec<Vec<String>> = Vec::new();

    for file_path in args.input {
        let json_str = fs::read_to_string(&file_path)?;

        let json_data = serde_json::from_str(&json_str)?;

        if let Value::Object(map) = json_data {
            for (_, field_value) in map {
                if let Value::Array(arr) = field_value {
                    let values: Vec<String> = arr
                        .iter()
                        .filter_map(|value| {
                            if let Value::String(s) = value {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    result.push(values);
                }
            }
        }
    }

    let wrapped_result: Vec<Vec<String>> = result;
    // println!("{:?}", wrapped_result);
    let combined_result = combinations_helper(&wrapped_result, 0);
    //println!("{:?}", combined_result);

    let mut writer = Writer::from_path(output_file)?;
    for row in combined_result {
        let row_as_string = row.join("");
        writer.write_record(&[row_as_string])?;
    }

    Ok(())
}
