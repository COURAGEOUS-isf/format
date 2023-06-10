use std::{
    fs::File,
    io::{BufWriter, Write},
};

use schema::Document;
use schemars::schema_for;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Document);
    let string = serde_json::to_string(&schema)?;
    BufWriter::new(File::create("courageous.schema.json")?).write_all(string.as_bytes())?;

    Ok(())
}
