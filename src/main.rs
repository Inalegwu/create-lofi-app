use std::{panic, path::PathBuf};

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

    // name

    let working_dir = match args.folder_name {
        Some(name) => PathBuf::new(),
        None => {
            let result = std::env::current_dir();

            match result {
                Ok(path) => path,
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
            println!("ElectronReactTs");
        }
        Template::ElectronSolidTs => {
            println!("{:#?}", working_dir);
        }
        Template::ReactNativeTs => {
            println!("ReactNativeTs");
        }
    }
}
