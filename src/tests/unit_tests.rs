/*
SUMMARY:    Unit Tests Dedicated for `storage.rs`

DIRECTORIES CREATED:
     (Windows 10/11)
         C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config
         C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data
         C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\templates
*/
mod unit_tests {
    use crate::components::storage::AppStorage;

    // Memory Drop Functionality to allow for independent functional testing
    impl Drop for AppStorage {
        fn drop(&mut self) {
            todo!()
        }
    }

    #[test]
    fn test_app_storage() {
        AppStorage::build();
    }
}
