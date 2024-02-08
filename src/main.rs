use csv;
use std::error::Error;

/* In RUST we dont have expcetion so we use Result when working with errors.
With Box we are boxing the underlying dynamic error. */
fn read_file_from(path: &str) -> Result<(), Box<dyn Error>> {
    /* Creating a mutable reader which reads the content is the csv file
    with the help of the Reader method. The "?" after the path can be
    considered an alternative to match which is just little less verbose. */
    let mut reader = csv::Reader::from_path(path)?;

    /* Here we are iterating over the contents of the reader Line-by-Line
    or row-by-row and we use record to handle each result and print it out. */
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    // In case of success we say Ok(())
    Ok(())
}

fn main() {
    if let Err(e) = read_file_from("./customers.csv") {
        eprintln!("{}", e);
    }
}
