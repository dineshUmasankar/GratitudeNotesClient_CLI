mod storage_behavior_tests {
    
    /*
    SUMMARY:    Unit Tests Dedicated for `storage.rs`

    DIRECTORIES CREATED:
        (Windows 10/11)
            C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config
            C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data
            C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\templates

    NOTES:      Tests must be ran single-threaded!
    */
    use crate::components::storage::AppStorage;
    use std::fs;

    #[test]
    /// Tests if the application incrementally regens config directory
    fn test_regen_config_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        fs::remove_dir(&create_test_env.config_dir).expect("Could not delete config directory.");
        let test_pass = AppStorage::new().is_ok(); // Attempted Regeneration of config directory
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }

    #[test]
    /// Tests if the application incrementally regens data directory
    fn test_regen_data_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        fs::remove_dir(&create_test_env.data_dir).expect("Could not delete data directory.");
        let test_pass = AppStorage::new().is_ok(); // Attempted Regeneration of data directory
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }

    #[test]
    /// Tests if the application incrementally template directory
    fn test_regen_template_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        fs::remove_dir(&create_test_env.template_dir).expect("Could not delete template directory.");
        let test_pass = AppStorage::new().is_ok(); // Attempted Regeneration of template directory
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }

    #[test]
    /// Tests if files can be written into the config directory
    fn test_write_config_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        let test_file_path = &create_test_env.config_dir.join("config_test.txt");
        let test_file_contents = b"Configuration Test";

        let write_test_file = fs::write(test_file_path, test_file_contents).is_ok();
        let read_test_file = fs::read(test_file_path).is_ok();

        let test_pass = write_test_file && read_test_file;
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }

    #[test]
    /// Tests if files can be written into the data directory
    fn test_write_data_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        let test_file_path = &create_test_env.data_dir.join("data_test.txt");
        let test_file_contents = b"Data Test";

        let write_test_file = fs::write(test_file_path, test_file_contents).is_ok();
        let read_test_file = fs::read(test_file_path).is_ok();

        let test_pass = write_test_file && read_test_file;
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }

    #[test]
    /// Tests if files can be written into the template directory
    fn test_write_template_dir() {
        let create_test_env = AppStorage::new().expect("Test Environment Creation Failed!");

        let test_file_path = &create_test_env.template_dir.join("template_test.txt");
        let test_file_contents = b"Template Test";

        let write_test_file = fs::write(test_file_path, test_file_contents).is_ok();
        let read_test_file = fs::read(test_file_path).is_ok();

        let test_pass = write_test_file && read_test_file;
        
        create_test_env.destroy().expect("Test Environment Cleanup Failed!");
        
        assert!(test_pass);
    }
}
