use std::fmt::Error;
use std::time::Duration;
use crate::jobs::JobData;

pub(crate) fn handle(job: JobData) -> Result<(), Error> {
    std::thread::sleep(Duration::from_millis(2000));
    let number_1: usize = job.data["number_1"].parse().expect("Can't parse a number");
    let number_2: usize = job.data["number_2"].parse().expect("Can't parse a number");

    println!("Result: Sum of the number {} and {} is {}",
             number_1.to_string(),
             number_2.to_string(),
             (number_1 + number_2).to_string()
    );
    
    Ok(())
}
