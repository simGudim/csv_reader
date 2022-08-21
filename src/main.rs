mod transaction_processor;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate csv;


use transaction_processor::{
    transaction::Transaction,
    processor::Proccessor
};
use csv::{
    ByteRecord, 
    ReaderBuilder, 
    Trim,
    Writer
};

use std::ffi::OsString;
use std::fs::File;
use std::env;
use std::error::Error;
use std::io;
use std::process;

// returns the positional argument sent to this process. 
// If there are no positional arguments, then this returns an error.
fn get_numbered_arg(pos: usize) -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(pos) {
        None => Err(From::from("expected argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn process_transactions() -> Result<(), Box<dyn Error>> {
    // get the input file path 
    let input_file_path = get_numbered_arg(1)?;
    let file = File::open(input_file_path)?;

    // intiliaze reader and allocate memory for the record
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(true)
        .from_reader(file);
    let mut raw_record = ByteRecord::new();
    let headers = rdr.byte_headers()?.clone();

    // process transaction
    let mut processor = Proccessor::new();
    while rdr.read_byte_record(&mut raw_record)? {
        let transaction: Transaction = raw_record.deserialize(Some(&headers))?;
        processor.process_transaction(transaction);
    }

    // output to stdout
    let mut wtr = Writer::from_writer(io::stdout());
    processor.accounts_map
        .iter()
        .try_for_each(|(_, account)| wtr.serialize(account))?;

    wtr.flush()?;
    Ok(())

}


fn main() {
    match process_transactions() {
        Ok(_) => {
            eprintln!("Processing Complete!!! Time to get Shwifty!");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}