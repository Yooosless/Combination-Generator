use clap::Parser;
use csv::Reader;
use csv::StringRecord;
use handlebars::Handlebars;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;

use crate::combinations_generator::Cli;

pub fn convert_csv_to_txt(csv_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = Reader::from_path(csv_file_path)?;

    let headers_record: StringRecord = csv_reader.headers()?.clone();

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true); // Add this line to enable strict mode

    for (row_index, result) in csv_reader.records().enumerate() {
        let record = result?;

        let mut context = HashMap::new();
        for (index, field) in record.iter().enumerate() {
            let field_name = headers_record[index].to_string();
            context.insert(field_name, field);
        }

        let rendered_content = handlebars.render_template(
            "Student: {{student}}, Teacher: {{teacher}}, Aya: {{aya}}, Date: {{date}}, Time: {{time}}",
            &context,
        )?;

        let args = Cli::parse();
        let output_file_name = format!("{:0>8}-{}.txt", row_index, args.file);
        //replace output as args
        let mut output_file = fs::File::create(output_file_name)?;

        writeln!(output_file, "{}", rendered_content)?;
    }

    Ok(())
}
