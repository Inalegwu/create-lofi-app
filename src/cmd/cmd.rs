use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Template {
    ElectronReactTs,
    ElectronSolidTs,
    ReactNativeTs,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum PackageManager {
    Pnpm,
    Npm,
    Bun,
    Yarn,
}

#[derive(Debug, Parser)]
#[command(version,about,long_about=None)]
pub struct Args {
    pub template: Template,
    pub folder_name: Option<String>,
    pub package_manager: Option<PackageManager>,
}