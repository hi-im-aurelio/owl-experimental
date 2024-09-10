use clap::{command, Arg, ArgMatches, Command};

/*
    1. owl init, comando para criar o clone de um repositorio git
        - deve receber project_path (a pasta do repositorio git)
        - deve pegar o nome da pasta do repositorio git, e criar essa mesma pasta em .owl/clones

    2. owl clone --list, deve lista as pastas (ou clones), na pasta .owl/clones
    3. owl clone --configure-remote, deve configurar a orgin
    4. owl guard --clone <clone-name>, deve iniciar o processo de cron no segundo plano.
    5. owl guard --look-up, deve listar todos processos relacionado ao owl, ou seja, todos os processos que estão a correr em segundo plano, ou que esteja a ser vigiados
    6. owl guard --unbind <clone-name> deve remover o processo relacionado ao clone que esteja a correr em segundo plano.

*/

fn main() {
    let match_result: ArgMatches = command!()
        .version("1.0.1")
        .subcommand(
            Command::new("init")
                .about("Initializes a copy of the original repository, locally and remotely")
                .arg(
                Arg::new("project-path")
                    .short('p')
                    .long("project-path")
                    .help("The git repository folder. .owlignore files should select all files that will not be cloned")
                    .required(true)
                    .aliases(["ppath", "project-path"]),
            ),
        )
        .subcommand(
            Command::new("clone")
                .about("Initialize a clone and manage a clone.")
                .arg(
                    Arg::new("list")
                        .long("list")
                        .short('l')
                        .help("Lists all clones made by owl. You can find these repositories in: $HOME/.clones/.")
                        .aliases(["list"]),
                )
                .arg(
                    Arg::new("configure-remote")
                        .short('C')
                        .long("configure-remote")
                        .help("Configures the remote source, between the local clone and the remote clone.")
                        .aliases(["remote"]),
                )
        )
        .subcommand(
            Command::new("guard")
                .about("Manipulate a running `guard`.")
                .arg(
                    Arg::new("clone-name")
                        .long("clone")
                        .short('r')
                        .help("Initialize a `guard` in a clone background.")
                )
                .arg(
                    Arg::new("look-up")
                        .short('L')
                        .long("look-up")
                        .help("List all `guards` (processes) related to owl, i.e. those that are being watched (in the background).")
                )
                .arg(
                    Arg::new("unbind")
                            .short('U')
                            .long("unbind")
                            .help("Removes a `guard` (process) related to the currently running clone.")
                )

        )
        .about("A Rust-based tool designed to automate the backup and synchronization of local Git repositories.")
        .get_matches();

    if let Some(matches) = match_result.subcommand_matches("init") {
        if let Some(path) = matches.get_one::<String>("project-path") {
            if path.is_empty() {
                println!("please, you need provide a path to initalize a clone");
            } else {
                println!("Provide path: {}", path);
            }
        }
    } else if let Some(matches) = match_result.subcommand_matches("clone") {
        if let Some(_) = matches.get_one::<String>("list") {
            println!("listing you clones...");
        } else if let Some(remote_address) = matches.get_one::<String>("configure-remote") {
            if remote_address.is_empty() {
                println!("you need provide a remote value");
            } else {
                println!("configuring your remote address: {}", remote_address);
            }
        } else {
            println!("???");
        }
    } else if let Some(x) = match_result.subcommand_matches("guard") {
        if let Some(clone) = x.get_one::<String>("clone-name") {
            if clone.is_empty() {
                println!("you need provide a clone to initialize a gruard");
            } else {
                println!("you clone is founded...");
            }
        } else if let Some(clone) = x.get_one::<String>("look-up") {
            if clone.is_empty() {
                println!("you need provide a clone to initialize a gruard");
            } else {
                println!("you clone is up now...");
            }
        } else if let Some(clone) = x.get_one::<String>("unbind") {
            if clone.is_empty() {
                println!("you need provide a clone ");
            } else {
                println!("you clone is down now...");
            }
        } else {
            println!("how do you want to proceed?");
        }
    }
}
