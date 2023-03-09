//! Data structures for the client.
//!
//! This module contains the data structures used by the client.

/// Data for a registration.
///
/// This struct contains the data defined at registration time.
#[derive(Clone, Debug, Default)]
pub struct RegistrationData {
    /// The nickname of the user
    pub nickname: String,
    /// The username of the user
    pub username: String,
    /// The realname of the user
    pub realname: String,
    /// The password of the user
    pub hostname: String,
    /// The servername of the user
    pub servername: String,
}
