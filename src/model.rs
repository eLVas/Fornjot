use std::{collections::HashMap, io, path::PathBuf, process::Command};

use thiserror::Error;

pub struct Model {
    name: String,
}

impl Model {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> String {
        format!("models/{}", self.name)
    }

    pub fn src_path(&self) -> PathBuf {
        format!("{}/src", self.path()).into()
    }

    pub fn load(
        &self,
        arguments: &HashMap<String, String>,
    ) -> Result<fj::Shape, Error> {
        let status = Command::new("cargo")
            .arg("build")
            .args(["--manifest-path", &format!("{}/Cargo.toml", self.path())])
            .status()?;

        if !status.success() {
            return Err(Error::Compile);
        }

        // TASK: Read up why those calls are unsafe. Make sure calling them is
        //       sound, and document why that is.
        let shape = unsafe {
            let lib = libloading::Library::new({
                let path = format!("{}/target/debug/", self.path(),);
                let filename: String;
                if cfg!(windows) {
                    filename = format!("{}.dll", self.name(),)
                } else if cfg!(target_os = "macos") {
                    filename = format!("lib{}.dylib", self.name(),)
                } else {
                    //Unix
                    filename = format!("lib{}.so", self.name(),)
                }
                format!("{}{}", path, filename)
            })?;
            let model: libloading::Symbol<ModelFn> = lib.get(b"model")?;
            model(&arguments)
        };

        Ok(shape)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error compiling model")]
    Compile,

    #[error("I/O error while loading model")]
    Io(#[from] io::Error),

    #[error("Error loading model from dynamic library")]
    LibLoading(#[from] libloading::Error),
}

type ModelFn =
    unsafe extern "C" fn(args: &HashMap<String, String>) -> fj::Shape;
