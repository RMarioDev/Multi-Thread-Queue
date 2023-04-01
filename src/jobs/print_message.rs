use std::fmt::Error;
use std::time::Duration;
use crate::jobs::JobData;

pub(crate) fn handle(job: JobData) -> Result<(), Error> {
    std::thread::sleep(Duration::from_millis(2000));
    println!("Result: {}", job.data["message"]);
    
    Ok(())
}
