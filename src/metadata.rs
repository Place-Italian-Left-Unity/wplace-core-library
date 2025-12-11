use std::path::PathBuf;

use serde::Deserialize;

use crate::template_data::{TemplateData, TemplateDataError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateMetadataDecode {
    name: String,
    file_name: String,
    coords: String,
    location: String,
    // TODO: Add alliances
    _alliance: String,
}

#[derive(thiserror::Error, Debug)]
pub enum TemplateMetadataDecodeError {
    #[error("Path doesn't exist")]
    InvalidPath,
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde Deserialize Error: {0}")]
    DeserializeError(#[from] serde_json::Error),
    #[error("TemplateData Error: {0}")]
    TemplateDataError(#[from] TemplateDataError),
}

fn normalize_path_for_file<P: AsRef<std::path::Path>, T: AsRef<std::path::Path>>(
    path: P,
    file: T,
) -> Result<PathBuf, TemplateMetadataDecodeError> {
    let path = path.as_ref();
    if !path.exists() || path.is_file() || path.is_symlink() {
        println!("Fail here");
    }

    let path = path.join(file);
    if !path.exists() || path.is_dir() || path.is_symlink() {
        return Err(TemplateMetadataDecodeError::InvalidPath);
    }

    return Ok(path);
}

impl TemplateMetadataDecode {
    pub fn from_template_folder_path<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Vec<Self>, TemplateMetadataDecodeError> {
        let path = normalize_path_for_file(path, "template_metadata.json")?;

        serde_json::from_reader(std::fs::File::open(path)?).map_err(Into::into)
    }

    pub fn get_image_path<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<std::path::PathBuf, TemplateMetadataDecodeError> {
        normalize_path_for_file(path, &self.file_name)
    }

    pub fn into_template<P: AsRef<std::path::Path>>(
        self,
        path: P,
    ) -> Result<TemplateData, TemplateMetadataDecodeError> {
        let path = self.get_image_path(path)?;
        TemplateData::from_data(self.name, &self.coords, path, Some(self.location))
            .map_err(Into::into)
    }
}
