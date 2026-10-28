use common::model::status_message::StatusMessage;
use std::fs;
use std::path::{Path, PathBuf};
use tetrislib::dto::model::Layout;
use uuid::Uuid;

/// Each solution owns exactly one source and at most one processed result.
pub struct SolutionFiles {
    root: PathBuf,
}

impl Default for SolutionFiles {
    fn default() -> Self {
        Self {
            root: PathBuf::from("/data"),
        }
    }
}

impl SolutionFiles {
    pub fn directory(&self, id: Uuid) -> PathBuf {
        self.root.join(id.to_string())
    }

    pub fn source(&self, id: Uuid) -> PathBuf {
        self.directory(id).join("source.json")
    }

    pub fn result(&self, id: Uuid) -> PathBuf {
        self.directory(id).join("result.json")
    }

    pub fn create(&self, id: Uuid) -> Result<(), StatusMessage> {
        fs::create_dir_all(&self.root).map_err(storage_error)?;
        // Never reuse another solution's directory during creation.
        fs::create_dir(self.directory(id)).map_err(storage_error)
    }

    pub fn parse_source(bytes: &[u8]) -> Result<Layout, StatusMessage> {
        serde_json::from_slice(bytes).map_err(|error| {
            StatusMessage::make_bad_request_message(&format!("File is not a Layout: {error}"))
        })
    }

    pub fn save_source(&self, id: Uuid, layout: &Layout) -> Result<(), StatusMessage> {
        // Legacy solutions may have been created before directory provisioning.
        fs::create_dir_all(self.directory(id)).map_err(storage_error)?;
        let bytes = serde_json::to_vec_pretty(layout).map_err(|error| {
            eprintln!("Cannot serialize solution layout: {error}");
            StatusMessage::make_internal_server_error_message("Cannot serialize layout")
        })?;
        let temporary = self
            .directory(id)
            .join(format!(".source-{}.tmp", Uuid::new_v4()));
        let write = (|| {
            fs::write(&temporary, bytes)?;
            // A result computed from the old input is no longer valid.
            match fs::remove_file(self.result(id)) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
            fs::rename(&temporary, self.source(id))
        })();
        if write.is_err() {
            let _ = fs::remove_file(temporary);
        }
        write.map_err(storage_error)
    }

    pub fn read(&self, id: Uuid, output: bool) -> Result<Layout, StatusMessage> {
        let path = if output {
            self.result(id)
        } else {
            self.source(id)
        };
        Self::read_layout(&path)
    }

    fn read_layout(path: &Path) -> Result<Layout, StatusMessage> {
        let bytes = fs::read(path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                StatusMessage::make_not_found_message("Solution layout is not available yet")
            } else {
                storage_error(error)
            }
        })?;
        serde_json::from_slice(&bytes).map_err(|error| {
            eprintln!("Invalid stored layout at {}: {error}", path.display());
            StatusMessage::make_internal_server_error_message("Stored file is not a valid Layout")
        })
    }
}

fn storage_error(error: std::io::Error) -> StatusMessage {
    eprintln!("Solution storage error: {error}");
    StatusMessage::make_internal_server_error_message("Cannot access solution storage")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{ResponseError, http::StatusCode};

    #[test]
    fn validates_and_stores_one_typed_source_and_optional_result() {
        let root = std::env::temp_dir().join(format!("tetris-files-{}", Uuid::new_v4()));
        let files = SolutionFiles { root: root.clone() };
        let id = Uuid::new_v4();
        files.create(id).unwrap();
        assert!(files.directory(id).is_dir());
        assert!(files.create(id).is_err());
        assert_eq!(
            files.read(id, false).unwrap_err().status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            files.read(id, true).unwrap_err().status_code(),
            StatusCode::NOT_FOUND
        );
        for bytes in [b"not json".as_slice(), b"{}", b"[]", br#"{"design": {}}"#] {
            assert_eq!(
                SolutionFiles::parse_source(bytes)
                    .unwrap_err()
                    .status_code(),
                StatusCode::BAD_REQUEST
            );
        }
        let source = include_bytes!("../../../GUI/examples/layout.json");
        let layout = SolutionFiles::parse_source(source).unwrap();
        files.save_source(id, &layout).unwrap();
        assert_eq!(files.read(id, false).unwrap(), layout);
        fs::write(files.result(id), serde_json::to_vec(&layout).unwrap()).unwrap();
        assert_eq!(files.read(id, true).unwrap(), layout);
        // Invalid uploads are rejected before touching either existing file.
        assert!(SolutionFiles::parse_source(b"{}").is_err());
        assert!(files.result(id).exists());
        let mut replacement = layout.clone();
        replacement.cells.clear();
        files.save_source(id, &replacement).unwrap();
        assert_eq!(files.read(id, false).unwrap(), replacement);
        assert!(!files.result(id).exists());
        assert_eq!(fs::read_dir(files.directory(id)).unwrap().count(), 1);
        fs::remove_dir_all(root).unwrap();
    }
}
