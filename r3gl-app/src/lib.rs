use std::{path::Path, fs::{OpenOptions, self}, io::Write, env};

use log::{error, warn};
use serde::{de::DeserializeOwned, Serialize};

pub mod state;
pub mod screen;
pub mod project;
pub mod view;
pub mod editor;
pub mod beatmap;
pub mod graphics;
pub mod identifier;
pub mod store;
pub mod unit;

pub fn save<T: Serialize>(obj: &T, path: impl AsRef<Path>) {
    let path = path.as_ref();
    if let Ok(mut file) = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(&path) {
        if file.write_all(toml::to_string(&obj).unwrap().as_bytes()).is_err() {
            warn!("Failed to save {:?}", &path);
        }
  } else {
      warn!("Failed to open {:?}", &path);
  }
}

pub fn load_or_default<T: DeserializeOwned + Serialize + Default>(path: impl AsRef<Path>) -> T {
    let path = env::current_dir().unwrap().join(path.as_ref());
    if let Ok(str) = fs::read_to_string(&path) {
        if let Ok(obj) = toml::from_str(&str) {
            return obj;
        } else {
            error!("Failed to parse {:?}, falling back to default", &path);
        }
    } else {
        let obj = T::default();
        if let Ok(mut file) = OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open(&path) {
            if file.write_all(toml::to_string(&obj).unwrap().as_bytes()).is_err() {
                warn!("Failed to save {:?}", &path);
            }
        } else {
            warn!("Failed to open {:?}", &path);
        }

        return obj;
    }

    return T::default();
}