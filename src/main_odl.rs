mod commands;
// mod utils;
use clap::{Arg, Command};
use commands::owl_init::run;

// use utils::copy_files::copy_files;
// use utils::read_owlignore::read_owlignore;

// use std::io;
// use std::path::Path;

// fn main() -> io::Result<()> {
//     let ignore_patterns = read_owlignore()?;
//     let source_dir = Path::new("/home/ivy/srv/work/piminder/api/microservices/authentication");
//     let destination_dir = Path::new("/home/ivy/.owl/clones/authentication");

//     copy_files(source_dir, destination_dir, &ignore_patterns)?;

//     println!("Arquivos copiados com sucesso!");

//     Ok(())
// }

// fn main() {
//     let clone_path = "/home/ivy/.owl/clones/authentication";
//     initialize_repo(clone_path);
// }

// fn main() {
//     let clone_path = "/home/ivy/.owl/clones/authentication";
//     let schedule = "0 0 * * * *"; // Substitua pela expressão cron desejada
//     run(clone_path, schedule);

//     //

//     // let repo_path = "/home/ivy/.owl/clones/authentication";
//     // let remote_url = "https://github.com/fariosofernando/authentication.git";

//     // // Chama a função para verificar e adicionar o remote "origin" se necessário
//     // if let Err(e) = add_remote_origin_if_not_exists(repo_path, remote_url) {
//     //     eprintln!("Erro: {}", e);
//     // }
// }

fn main() {
    let matches = Command::new("owl")
        .version("1.0")
        .about("Owl CLI for managing projects")
        .subcommand_required(true)
        .subcommand(Command::new("init").about("Initialize the project"))
        .subcommand(Command::new("run").about("Run the owl process"))
        .subcommand(
            Command::new("add-origin")
                .about("Add a new origin to the project")
                .arg(
                    Arg::new("project")
                        .about("Name of the project directory")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("url")
                        .about("URL of the new remote origin")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            // Implementar a lógica para `owl init`
            println!("Initializing project...");
        }
        Some(("run", _)) => {
            // Implementar a lógica para `owl run`
            println!("Running owl...");
        }
        Some(("add-origin", sub_m)) => {
            let project = sub_m.value_of("project").unwrap();
            let url = sub_m.value_of("url").unwrap();
            // Implementar a lógica para adicionar origin
            println!("Adding new origin '{}' to project '{}'", url, project);
        }
        _ => unreachable!("No such command"),
    }
}
