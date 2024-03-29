use std::path;

use anyhow::Context;
use tauri::AppHandle;

use crate::storage;

use super::{storage::Storage, PrivateKey};

#[derive(Clone)]
pub struct Controller {
    storage: Storage,
}

impl From<&path::PathBuf> for Controller {
    fn from(value: &path::PathBuf) -> Self {
        Self {
            storage: Storage::from(value),
        }
    }
}

impl From<&storage::Storage> for Controller {
    fn from(value: &storage::Storage) -> Self {
        Self {
            storage: Storage::from(value),
        }
    }
}

impl From<&AppHandle> for Controller {
    fn from(value: &AppHandle) -> Self {
        Self {
            storage: Storage::from(value),
        }
    }
}

impl Controller {
    pub fn new(storage: &Storage) -> Self {
        Self {
            storage: storage.clone(),
        }
    }

    pub fn get_or_create(&self) -> Result<PrivateKey, GetOrCreateError> {
        if let Some(key) = self.storage.get().context("failed to get key")? {
            Ok(key)
        } else {
            let key = PrivateKey::generate();
            self.storage.create(&key).context("failed to save key")?;
            Ok(key)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GetOrCreateError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[cfg(not(target_os = "windows"))]
#[cfg(test)]
mod tests {
    use std::fs;
    #[cfg(target_family = "unix")]
    use std::os::unix::prelude::*;

    use crate::test_utils::Suite;

    use super::*;

    #[test]
    fn test_get_or_create() {
        let suite = Suite::default();
        let controller = Controller::from(&suite.local_app_data);

        let once = controller.get_or_create().unwrap();
        let twice = controller.get_or_create().unwrap();
        assert_eq!(once, twice);

        // check permissions of the private key
        let permissions = fs::metadata(suite.local_app_data.join("keys/ed25519"))
            .unwrap()
            .permissions();
        let perms = format!("{:o}", permissions.mode());
        assert_eq!(perms, "100600");
    }
}
