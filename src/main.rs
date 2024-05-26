use std::{panic, path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Template {
    ElectronReactTs,
    ElectronSolidTs,
    ReactNativeTs,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum PackageManager {
    Pnpm,
    Npm,
    Bun,
    Yarn,
}

#[derive(Debug, Parser)]
#[command(version,about,long_about=None)]
struct Args {
    template: Template,
    folder_name: Option<String>,
    package_manager: Option<PackageManager>,
}

fn main() {
    let args = Args::parse();

    let pkg_man = match args.package_manager {
        None => match args.template {
            Template::ElectronReactTs => PackageManager::Pnpm,
            Template::ElectronSolidTs => PackageManager::Pnpm,
            Template::ReactNativeTs => PackageManager::Bun,
        },
        Some(p_man) => match p_man {
            PackageManager::Bun => match args.template {
                Template::ElectronReactTs => panic!("Can't use bun with electron"),
                Template::ElectronSolidTs => panic!("Can't use bun with electron"),
                Template::ReactNativeTs => p_man,
            },
            PackageManager::Npm => p_man,
            PackageManager::Pnpm => p_man,
            PackageManager::Yarn => p_man,
        },
    };

    let working_dir = match args.folder_name {
        Some(name) => {
            let result = PathBuf::from_str(name.as_str());

            let full_url = match result {
                Ok(path) => {
                    let current_dir = match std::env::current_dir() {
                        Ok(dir) => dir,
                        Err(err) => {
                            println!("{:#?}", err);
                            panic!("working directory error constructing full path")
                        }
                    };

                    let destination = format!("{:#?}/{:#?}", current_dir, path);
                    destination
                }
                Err(err) => {
                    println!("{:#}", err);
                    panic!("Couldn't work with current directory");
                }
            };

            full_url
        }
        None => {
            let result = std::env::current_dir();

            match result {
                Ok(path) => {
                    let full_url = format!("{:#?}", path);
                    full_url
                }
                Err(err) => {
                    println!("{:#}", err);
                    panic!("Couldn't work with current directory");
                }
            }
        }
    };

    println!("{:#?}", pkg_man);

    match args.template {
        Template::ElectronReactTs => {
            println!("ElectronReactTs: {:#?}", working_dir);
        }
        Template::ElectronSolidTs => {
            println!("{:#?}", working_dir);
        }
        Template::ReactNativeTs => {
            println!("ReactNativeTs: {:#?}", working_dir);
        }
    }
}
