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
    let _match_result: ArgMatches = command!()
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

    // // let pet_args = match_result.subcommand_matches("register-pet"); // let person = match_result.subcommand_matches("register-person"); // if person.unwrap().contains_id("firstname") { //     println!( //         "Firstname: {}", //         person.unwrap().get_one::<String>("firstname").unwrap() //     ); // } // if person.unwrap().contains_id("lastname") { //     println!( //         "Lastname: {}", //         person.unwrap().get_one::<String>("lastname").unwrap() //     ); // }
    // println!(
    //     "Does pet name exist? {}",
    //     pet_args.unwrap().contains_id("petname")
    // );

    // println!(
    //     "Does pet name exist? {}",
    //     pet_args.unwrap().get_one::<String>("petname").unwrap()
    // );

    // if pet_args.unwrap().contains_id("pet-name") {
    //     println!("Does pet name exist? {}", true);
    // }
}
