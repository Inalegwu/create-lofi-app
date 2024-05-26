use std::{ffi::OsStr, fs, path::Path};

use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
enum Template {
    ElectronReactTs,
    ElectronSolidTs,
    ReactNativeTs,
}

#[derive(Debug, Parser)]
#[command(version,about,long_about=None)]
struct Args {
    template: Template,
    folder_name: String,
}

fn main() {
    let args = Args::parse();

    match args.template {
        Template::ElectronReactTs => {
            println!("ElectronReactTs");
        }
        Template::ElectronSolidTs => {
            let _ = fs::create_dir_all(&args.folder_name);

            let result = fs::copy(
                "/src/templates/electron-solid-ts/",
                OsStr::new(args.folder_name),
            );

            match result {
                Ok(_) => {
                    println!("Successful")
                }
                Err(err) => {
                    println!("{:#?}", err);
                    panic!("Didn't work")
                }
            }
        }
        Template::ReactNativeTs => {
            println!("ReactNativeTs");
        }
    }
}
