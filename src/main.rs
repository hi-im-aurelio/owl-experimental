use clap::{command, Arg, ArgMatches, Command};
use std::fmt::format;
use std::io;
use std::path::Path;

mod commands;
mod utils;

fn main() -> io::Result<()> {
    let owl_path = utils::get_owl_path::owl_path();

    let match_result: ArgMatches = command!()
        .version("1.0.0")
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
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Empty project path",
                ));
            } else {
                let ignore_patterns = utils::read_owlignore::read_owlignore()?;

                // obetendo o nome do projeto (ultima parte do caminho)
                let project_name = Path::new(path)
                    .file_name()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid project path")
                    })?
                    .to_str()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Invalid project name")
                    })?;

                let des = owl_path.join(format!("{}/{}", "clones", project_name));
                commands::clone::clone(Path::new(path), des.as_path(), &ignore_patterns)?;

                println!(
                    "Your clone was created at: {}/ You can use `owl clone --list`, to view.",
                    des.display()
                );
            }
        }
    } else if let Some(matches) = match_result.subcommand_matches("clone") {
        if let Some(_) = matches.get_one::<String>("list") {
            println!("listing you clones...");
        } else if let Some(remote_address) = matches.get_one::<String>("configure-remote") {
            if remote_address.is_empty() {
                println!("you need provide a remote value");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Empty remote address",
                ));
            } else {
                println!("configuring your remote address: {}", remote_address);
            }
        } else {
            println!("???");
        }
    } else if let Some(x) = match_result.subcommand_matches("guard") {
        if let Some(clone) = x.get_one::<String>("clone-name") {
            if clone.is_empty() {
                println!("you need provide a clone to initialize a guard");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Empty clone name",
                ));
            } else {
                println!("your clone is found...");
            }
        } else if let Some(clone) = x.get_one::<String>("look-up") {
            if clone.is_empty() {
                println!("you need provide a clone to look up");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Empty clone name",
                ));
            } else {
                println!("your clone is up now...");
            }
        } else if let Some(clone) = x.get_one::<String>("unbind") {
            if clone.is_empty() {
                println!("you need provide a clone");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Empty clone name",
                ));
            } else {
                println!("your clone is down now...");
            }
        } else {
            println!("how do you want to proceed?");
        }
    }

    Ok(())
}
