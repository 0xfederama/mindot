use clap::{Parser, Subcommand};
use clean_path::Clean;
use dirs;
use fs_extra::dir;
use std::fs::{self, File};
use std::path::PathBuf;
use text_io::read;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add files or directories to the list
    Add {
        /// List of files to add
        file_list: Vec<String>,
    },

    /// Delete files or directories from the list
    Del {
        /// List of files to delete
        file_list: Vec<String>,
    },

    /// Backup the files, copying from the home directory to path
    Backup {
        /// Path in which to copy the files
        path: String,
    },

    /// Restore the files from cwd to the home directory
    Restore,

    /// List the files to back up
    List,
}

fn get_full_path(path: String) -> PathBuf {
    let mut path_prefix = PathBuf::default();
    if !path.contains("~") {
        let curr_path = std::env::current_dir().expect("Unable to get current path");
        path_prefix = path_prefix.join(curr_path);
    }
    let full_path: PathBuf = path_prefix.join(&*path).clean();
    return full_path;
}

fn main() {
    let cli = Cli::parse();

    let mut dotfiles: Vec<PathBuf> = Vec::new();

    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let config_dir = &home_dir.join(".config");

    let mindot_config_dir = config_dir.join("mindot");
    let mindot_config_file_path = mindot_config_dir.join("files.json");
    if !mindot_config_file_path.exists() {
        if !mindot_config_dir.exists() {
            fs::create_dir(config_dir.join("mindot")).expect("Unable to create directory");
        }
        File::create(&mindot_config_file_path).expect("Unable to create file");
    } else {
        let textfile = std::fs::read_to_string(&mindot_config_file_path).unwrap();
        dotfiles = serde_json::from_str::<Vec<_>>(&textfile).unwrap_or(Vec::new());
    }

    // Copy options for fs_extra::copy_items
    let mut copy_options = dir::CopyOptions::new();
    copy_options.overwrite = true;

    match &cli.command {
        Commands::Add { file_list } => {
            for path in file_list {
                let full_path = get_full_path(path.to_string());
                if !full_path.exists() {
                    eprintln!("{path:?} doesn't exist");
                    std::process::exit(1);
                }
                if !dotfiles.contains(&full_path) {
                    dotfiles.push(full_path);
                } else {
                    eprintln!("{path:?} is already in");
                    std::process::exit(1);
                }
                println!("{path:?} added");
            }
            std::fs::write(
                mindot_config_file_path,
                serde_json::to_string_pretty(&dotfiles).unwrap(),
            )
            .unwrap();
        }
        Commands::Del { file_list } => {
            for path in file_list {
                let full_path = get_full_path(path.to_string());
                if let Some(pos) = dotfiles.iter().position(|x| *x == full_path) {
                    dotfiles.remove(pos);
                } else {
                    println!("{path:?} has not been added before");
                    std::process::exit(1);
                }
                println!("{path:?} deleted");
            }
            std::fs::write(
                mindot_config_file_path,
                serde_json::to_string_pretty(&dotfiles).unwrap(),
            )
            .unwrap();
        }
        Commands::Backup { path } => {
            let full_path = get_full_path(path.to_string());
            if !full_path.is_dir() {
                eprintln!("You have to specify a directory to place the files in");
                std::process::exit(1);
            }
            println!("Backing up all the files to {path:?}");
            let mut to_copy = Vec::new();
            for f in dotfiles.iter() {
                to_copy.push(f);
            }
            fs_extra::copy_items(&to_copy, full_path, &copy_options).unwrap();
        }
        Commands::Restore => {
            let mut to_copy = Vec::new();
            for f in fs::read_dir(".").unwrap() {
                let file = f.as_ref().unwrap();
                let filename = file.file_name();
                // Read [yn]
                loop {
                    print!("Do you want to restore {filename:?} to home? [Y/n] ",);
                    let answer: String = read!("{}\n");
                    match answer.as_str() {
                        "y" | "Y" | "" => {
                            to_copy.push(filename);
                            break;
                        }
                        "n" | "N" => {
                            break;
                        }
                        _ => {
                            eprintln!("Not a valid response");
                        }
                    }
                }
            }
            fs_extra::copy_items(&to_copy, home_dir, &copy_options).unwrap();
        }
        Commands::List => {
            if !dotfiles.is_empty() {
                for f in dotfiles {
                    println!("{}", f.to_str().unwrap());
                }
            } else {
                println!("The dotfiles list is empty");
            }
        }
    }
}
