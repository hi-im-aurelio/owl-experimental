use cron::Schedule;
use git2::Repository;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

pub fn initialize_repo(clone_path: &str) {
    let repo_path = Path::new(clone_path);

    Command::new("git")
        .arg("init")
        .current_dir(repo_path)
        .status()
        .expect("Failed to initialize Git repository");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("initial commit")
        .arg("--allow-empty")
        .current_dir(repo_path)
        .status()
        .expect("Failed to commit");

    Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(repo_path.file_name().unwrap().to_str().unwrap())
        .arg("--private")
        .current_dir(repo_path)
        .status()
        .expect("Failed to create GitHub repository");

    println!("Git repository initialized and pushed to GitHub successfully!");
}

pub fn run(clone_path: &str, schedule: &str) {
    let repo_path = Path::new(clone_path);

    git_add_commit_push(repo_path);

    let schedule = Schedule::from_str(schedule).expect("Invalid cron schedule");

    for datetime in schedule.upcoming(chrono::Utc) {
        let wait_time = datetime - chrono::Utc::now();
        let wait_duration = wait_time.to_std().unwrap_or(Duration::new(0, 0));

        thread::sleep(wait_duration);

        git_add_commit_push(repo_path);
    }
}

fn git_add_commit_push(repo_path: &Path) {
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(repo_path)
        .status()
        .expect("Failed to add files to Git");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("owl: automated commit")
        .current_dir(repo_path)
        .status()
        .expect("Failed to commit");

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .current_dir(repo_path)
        .status()
        .expect("Failed to push to GitHub");
}

pub fn add_remote_origin_if_not_exists(
    repo_path: &str,
    remote_url: &str,
) -> Result<(), git2::Error> {
    // Abre o repositório no caminho especificado
    let repo = Repository::open(repo_path)?;

    // Verifica se o remote "origin" já existe
    if repo.find_remote("origin").is_err() {
        println!("Remote 'origin' não encontrado. Adicionando...");

        // Comando para adicionar o remote "origin"
        let output = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote_url)
            .current_dir(repo_path)
            .output()
            .expect("Falha ao adicionar remote origin");

        // Verifica se o comando foi executado com sucesso
        if output.status.success() {
            println!("Remote 'origin' adicionado com sucesso!");
        } else {
            eprintln!(
                "Erro ao adicionar remote 'origin': {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    } else {
        println!("Remote 'origin' já existe.");
    }

    Ok(())
}
