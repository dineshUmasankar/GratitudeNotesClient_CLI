/// Contains the application's local storage components, such as creation and regeneration
/// if the application's local storage components are missing.
/// 
/// # Errors
/// - Any OS-level errors are propagated up through as an `std::io::Error`
/// 
/// # File Structure
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\config`
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\data`
/// - `C:\Users\USERNAME\AppData\Roaming\gratitude-notes\GratitudeNotesClient_CLI\templates` 
pub mod storage;
