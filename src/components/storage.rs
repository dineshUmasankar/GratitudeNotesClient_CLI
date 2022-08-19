use std::{fs, io::ErrorKind, path::PathBuf};

/// Builds the path strings for the config, data, and template directory
struct PathBuilder {
    config_path: PathBuf,
    data_path: PathBuf,
    template_path: PathBuf,
}

impl PathBuilder {
    fn new() -> Result<PathBuilder, std::io::Error> {
        Ok(PathBuilder {
            config_path: Self::build_config_path()?,
            data_path: Self::build_data_path()?,
            template_path: Self::build_template_path()?,
        })
    }

    /// Builds the path string for the config directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config`
    fn build_config_path() -> Result<PathBuf, std::io::Error> {
        // Retrieves base path to store app files onto
        // Ex: C:\Users\USERNAME\AppData\Roaming
        let os_dir: Option<PathBuf> = dirs::config_dir();

        match os_dir {
            Some(mut config_dir) => {
                config_dir.push("gratitude-notes");
                config_dir.push("GratitudeNotesClient_CLI");
                config_dir.push("config");

                Ok(config_dir)
            }
            None => Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "Unsupported Filesystem! We cannot figure out the path for the config directory.",
            )),
        }
    }

    /// Builds the path string for the data directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data`
    fn build_data_path() -> Result<PathBuf, std::io::Error> {
        // Retrieves base path to store app files onto
        // Ex: C:\Users\USERNAME\AppData\Roaming
        let os_dir: Option<PathBuf> = dirs::config_dir();

        match os_dir {
            Some(mut data_dir) => {
                data_dir.push("gratitude-notes");
                data_dir.push("GratitudeNotesClient_CLI");
                data_dir.push("data");

                Ok(data_dir)
            }
            None => Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "Unsupported Filesystem! We cannot figure out the path for the data directory.",
            )),
        }
    }

    /// Builds the path string for the template directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\template`
    fn build_template_path() -> Result<PathBuf, std::io::Error> {
        // Retrieves base path to store app files onto
        // Ex: C:\Users\USERNAME\AppData\Roaming
        let os_dir: Option<PathBuf> = dirs::config_dir();

        match os_dir {
            Some(mut template_dir) => {
                template_dir.push("gratitude-notes");
                template_dir.push("GratitudeNotesClient_CLI");
                template_dir.push("template");

                Ok(template_dir)
            }
            None => Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "Unsupported Filesystem! We cannot figure out the path for the template directory.",
            )),
        }
    }
}

/// Verifies if the config directory already exists on the host system
struct ConfigIntegrity {
    config_dir: PathBuf,
    exists: bool,
}

impl ConfigIntegrity {
    fn new(config_dir: PathBuf) -> ConfigIntegrity {
        let exists = fs::read_dir(&config_dir).is_ok();

        ConfigIntegrity {
            config_dir,
            exists,
        }
    }
}

/// Verifies if the data directory already exists on the host system
struct DataIntegrity {
    data_dir: PathBuf,
    exists: bool,
}

impl DataIntegrity {
    fn new(data_dir: PathBuf) -> DataIntegrity {
        let exists = fs::read_dir(&data_dir).is_ok();

        DataIntegrity {
            data_dir,
            exists,
        }
    }
}

/// Verifies if the template directory already exists on the host system
struct TemplateIntegrity {
    template_dir: PathBuf,
    exists: bool,
}

impl TemplateIntegrity {
    fn new(template_dir: PathBuf) -> TemplateIntegrity {
        let exists = fs::read_dir(&template_dir).is_ok();

        TemplateIntegrity {
            template_dir,
            exists,
        }
    }
}

/// Based on the integrity checks, `FolderBuilder` will build the config,
/// data, and template directories as long as they don't already exist.
/// 
/// # Errors
/// Any OS-level Errors are propagated up through the Result as an std::io::Error.
struct FolderBuilder {}

impl FolderBuilder {
    fn new(storage_paths: PathBuilder) -> std::io::Result<AppStorage> {
        let config_integrity = ConfigIntegrity::new(storage_paths.config_path);
        let data_integrity = DataIntegrity::new(storage_paths.data_path);
        let template_integrity = TemplateIntegrity::new(storage_paths.template_path);

        if !config_integrity.exists {
            Self::create_config_dir(&config_integrity.config_dir)?;
        }

        if !data_integrity.exists {
            Self::create_data_dir(&data_integrity.data_dir)?;
        }

        if !template_integrity.exists {
            Self::create_template_dir(&template_integrity.template_dir)?;
        }

        Ok(AppStorage {
            config_dir: config_integrity.config_dir,
            data_dir: data_integrity.data_dir,
            template_dir: template_integrity.template_dir,
        })
    }

    fn create_config_dir(config_dir: &PathBuf) -> std::io::Result<()> {
        fs::create_dir_all(config_dir)
    }

    fn create_data_dir(data_dir: &PathBuf) -> std::io::Result<()> {
        fs::create_dir_all(data_dir)
    }

    fn create_template_dir(template_dir: &PathBuf) -> std::io::Result<()> {
        fs::create_dir_all(template_dir)
    }
}

/// Handles the application's local storage creation & regeneration, in three stages.
/// 
/// # Behavior
/// Creates the following file structure:
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config`
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data`
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\templates`
/// 
/// # Stages
/// - PathBuilder (Determines the path of where the app's local storage shall be)
/// - Integrity (Determines if that path already exists and prevents existing data from being overwritten)
/// - FolderBuilder (Builds folders from provided paths, as long as they don't already exist)
/// 
/// # Errors
/// - Any OS-level errors are propagated up through as an `std::io::Error`
pub struct AppStorage {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub template_dir: PathBuf,
}

impl AppStorage {
    pub fn new() -> Result<AppStorage, std::io::Error> {
        let app_paths = PathBuilder::new()?;
        let app_dirs = FolderBuilder::new(app_paths)?;

        Ok(app_dirs)
    }

    pub fn destroy(&self) -> std::io::Result<()> {
        // App directory will always exist as it's hardcoded ("GratitudeNotesClient_CLI")
        // Goes up from config directory into the app directory.
        let mut dir_parents = self.config_dir.ancestors(); // Gets parent folders of config directory
        dir_parents.next();
        
        let app_dir = dir_parents.next().unwrap(); // "GratitudeNotesClient_CLI" folder path
        fs::remove_dir_all(app_dir)?;
        Ok(())
    }
}