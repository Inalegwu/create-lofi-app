use clap::Parser;
use create_lofi_app::cmd::cmd::{Args, PackageManager, Template};
use create_lofi_app::decoder::decoder::Decoder;
use std::env;
use std::ffi::OsStr;
use std::panic;
use std::path::Path;

fn main() {
    let args = Args::parse();

    let pkg_man = match args.package_manager {
        None => match args.template {
            Template::ElectronReactTs => PackageManager::Pnpm,
            Template::ElectronSolidTs => PackageManager::Pnpm,
            Template::ReactNativeTs => PackageManager::Bun,
        },
        Some(manager) => match manager {
            PackageManager::Bun => match args.template {
                Template::ElectronReactTs => panic!("Can't use bun with electron"),
                Template::ElectronSolidTs => panic!("Can't use bun with electron"),
                Template::ReactNativeTs => manager,
            },
            PackageManager::Npm => manager,
            PackageManager::Pnpm => manager,
            PackageManager::Yarn => manager,
        },
    };

    let working_dir = match args.folder_name {
        Some(folder_name) => {
            let final_dest = OsStr::new(folder_name.as_str());

            let working_dir = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    panic!("{:#?}", err)
                }
            };

            let working_dir_path = Path::new(working_dir.as_path());

            let final_path = Path::new(final_dest);

            let final_dest_path = working_dir_path.join(final_path);

            final_dest_path
        }
        None => {
            let current_dir = match env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    panic!("{:#?}", err)
                }
            };
            current_dir
        }
    };

    let mut decoder = Decoder::new(working_dir, args.template, pkg_man);

    decoder.handle();
}
