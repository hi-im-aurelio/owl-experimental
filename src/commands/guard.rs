use cron::Schedule;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::utils;

fn execute_task() {
    // let ignore_patterns = utils::read_owlignore::read_owlignore();

    // let project_name = Path::new(path)
    //     .file_name()
    //     .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid project path"))?
    //     .to_str()
    //     .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid project name"))?;

    // let des = owl_path.join(format!("{}/{}", "clones", project_name));
    // commands::clone::clone(Path::new(path), des.as_path(), &ignore_patterns)?;

    // println!(
    //     "Your clone was created at: {}/ You can use `owl clone --list`, to view.",
    //     des.display()
    // );
}

pub fn guard() {
    let schedule = Schedule::from_str("0 8,14,20 * * *").expect("Invalid cron schedule");

    loop {
        let now = chrono::Utc::now();
        let upcoming = schedule
            .upcoming(chrono::Utc)
            .take(1)
            .next()
            .expect("No upcoming schedule found");

        let wait_time = upcoming - now;
        let wait_duration = wait_time.to_std().unwrap_or(Duration::new(0, 0));

        println!("Next execution at: {}", upcoming);

        thread::sleep(wait_duration);

        execute_task();
    }
}
