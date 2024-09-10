use cron::Schedule;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::commands::git_add_commit_push;

pub fn run(clone_path: &str, schedule: &str) {
    let repo_path = Path::new(clone_path);

    git_add_commit_push::git_add_commit_push(repo_path);

    let schedule = Schedule::from_str(schedule).expect("Invalid cron schedule");

    for datetime in schedule.upcoming(chrono::Utc) {
        let wait_time = datetime - chrono::Utc::now();
        let wait_duration = wait_time.to_std().unwrap_or(Duration::new(0, 0));

        thread::sleep(wait_duration);

        git_add_commit_push::git_add_commit_push(repo_path);
    }
}
