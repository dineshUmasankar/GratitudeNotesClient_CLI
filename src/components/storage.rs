use std::{
    io::{Error, ErrorKind},
    path::PathBuf
};


/**
The CLI Application requires local storage in order to store
offline notes, authorization tokens, and prompt templates.
<br></br>
`AppStorage` verifies whether the file structure below exists,
otherwise it will create it. It also allows for unit testing on
operations which require local application directory access / interactions.
<br></br>
It creates the application's file structure shown below and deletes them, when
the struct is dropped from memory, which allows for independent functional
testing. (Memory Drop Functionality only implemented within `unit_tests.rs`).
<br></br>
File Structure</br>
`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config`
`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data`
`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\templates`
*/  
pub struct AppStorage {
    project_dir: PathBuf,
    config_dir: PathBuf,
    data_dir: PathBuf,
    template_dir: PathBuf,
}

impl AppStorage {
    // pub fn verify() -> bool {
    //     true
    // }

    /// Path String Builder, which builds all of the required directories for the application to locally save data.
    pub fn build() -> Result<AppStorage, Error> {

        let storage_builder = AppStorage {
            project_dir: set_project_dir()?,
            config_dir: set_config_dir()?,
            data_dir: set_data_dir()?,
            template_dir: set_template_dir()?,
        };

        Ok(storage_builder)
    }

    pub fn create(&self) {

    }

    // pub fn destroy(&self) {

    //     // 2nd destroy the files
    // }
}

/// Builds the path string for the project directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\`
fn set_project_dir() -> Result<PathBuf, std::io::Error> {
    let os_dir: Option<PathBuf> = dirs::config_dir();

    match os_dir {
        Some(mut project_dir) => {
            project_dir.push("gratitude-notes");
            project_dir.push("GratitudeNotesClient_CLI");

            Ok(project_dir)
        },
        None => {
            Err(
                std::io::Error::new(
                    ErrorKind::Unsupported,
                    "Unsupported Filesystem! We cannot figure out where to generate the application directory."
                )
            )
        },
    }
}

///Builds the path string for the config directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config`
fn set_config_dir() -> Result<PathBuf, std::io::Error> {
    // Retrieves base path to store app files onto
    // Ex: C:\Users\USERNAME\AppData\Roaming
    let os_dir: Option<PathBuf> = dirs::config_dir();

    match os_dir {
        Some(mut config_dir) => {
            config_dir.push("gratitude-notes");
            config_dir.push("GratitudeNotesClient_CLI");
            config_dir.push("config");

            Ok(config_dir)
        },
        None => {
            Err(
                std::io::Error::new(
                    ErrorKind::Unsupported,
                    "Unsupported Filesystem! We cannot figure out where to generate the application's configuration directory."
                )
            )
        },
    }
}

/// Builds the path string for the data directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data`
fn set_data_dir() -> Result<PathBuf, std::io::Error> {
    // Retrieves base path to store app files onto
    // Ex: C:\Users\USERNAME\AppData\Roaming
    let os_dir: Option<PathBuf> = dirs::config_dir();

    match os_dir {
        Some(mut data_dir) => {
            data_dir.push("gratitude-notes");
            data_dir.push("GratitudeNotesClient_CLI");
            data_dir.push("data");

            Ok(data_dir)
        },
        None => {
            Err(
                std::io::Error::new(
                    ErrorKind::Unsupported,
                    "Unsupported Filesystem! We cannot figure out where to generate the application's data directory."
                )
            )
        },
    }
}

/// Builds the path string for the template directory: </br>`C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\template`
fn set_template_dir() -> Result<PathBuf, std::io::Error> {
    // Retrieves base path to store app files onto
    // Ex: C:\Users\USERNAME\AppData\Roaming
    let os_dir: Option<PathBuf> = dirs::config_dir();

    match os_dir {
        Some(mut template_dir) => {
            template_dir.push("gratitude-notes");
            template_dir.push("GratitudeNotesClient_CLI");
            template_dir.push("template");

            Ok(template_dir)
        },
        None => {
            Err(
                std::io::Error::new(
                    ErrorKind::Unsupported,
                    "Unsupported Filesystem! We cannot figure out where to generate the application's template's directory."
                )
            )
        },
    }
}