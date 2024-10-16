use csv_reader::read_csv;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let filename = "user.csv";
    read_csv(filename)


}
