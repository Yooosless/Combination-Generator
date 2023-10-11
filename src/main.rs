mod combinations_generator;
mod file_generator;

use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = combinations_generator::Cli::parse();
    combinations_generator::generate_combinations(&args)?;
    let output_file = &args.output;
    file_generator::convert_csv_to_txt(&output_file)?;
    Ok(())
}
