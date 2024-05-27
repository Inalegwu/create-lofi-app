use std::{
    fs, io,
    path::{Path, PathBuf},
};

use super::super::cmd::cmd::{PackageManager, Template};

pub struct Decoder {
    pub working_dir: PathBuf,
    pub template: Template,
    pub pkg_manager: PackageManager,
}

impl Decoder {
    pub fn new(working_dir: PathBuf, template: Template, p_manager: PackageManager) -> Self {
        return Decoder {
            pkg_manager: p_manager,
            template,
            working_dir,
        };
    }

    pub fn handle(&mut self) {
        match self.template {
            Template::ElectronReactTs => self.react_ts(),
            Template::ElectronSolidTs => self.solid_ts(),
            Template::ReactNativeTs => self.native_ts(),
        }
    }

    fn react_ts(&self) {
        println!("{:#?}", &self.working_dir.as_path());

        if self.working_dir.as_path().exists() {
            let _ = self.copy_dir_all(
                "../templates/electron-react-ts/",
                self.working_dir.as_path(),
            );
            return;
        }

        match fs::create_dir(self.working_dir.as_path()) {
            Ok(_) => {
                let _ = self.copy_dir_all(
                    "../templates/electron-react-ts/",
                    self.working_dir.as_path(),
                );

                println!("Success,finally");
            }
            Err(err) => {
                panic!("{:#?}", err)
            }
        }
    }

    fn solid_ts(&self) {
        println!("{:#?}", &self.working_dir);

        if self.working_dir.as_path().exists() {
            let _ = self.copy_dir_all(
                "../templates/electron-solid-ts/",
                self.working_dir.as_path(),
            );
            return;
        }

        match fs::create_dir(self.working_dir.as_path()) {
            Ok(_) => {
                let _ = self.copy_dir_all(
                    "../templates/electron-solid-ts/",
                    self.working_dir.as_path(),
                );

                println!("Success,finally");
            }
            Err(err) => {
                panic!("{:#?}", err)
            }
        }
    }

    fn native_ts(&self) {
        println!("{:#?}", &self.working_dir.as_path());

        if self.working_dir.as_path().exists() {
            let _ = self.copy_dir_all("../templates/react-native-ts/", self.working_dir.as_path());
            return;
        }

        match fs::create_dir(self.working_dir.as_path()) {
            Ok(_) => {
                let _ =
                    self.copy_dir_all("../templates/react-native-ts/", self.working_dir.as_path());

                println!("Success,finally");
            }
            Err(err) => {
                panic!("{:#?}", err)
            }
        }
    }

    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;

            let ty = entry.file_type()?;

            if ty.is_dir() {
                self.copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                let _ = fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }

        Ok(())
    }
}
