//! This module define [ClientData]

use gtk::glib;

#[derive(glib::Boxed, Clone, Debug, Default)]
#[boxed_type(name = "ClientData")]
/// Stores user data for connected client
pub struct ClientData {
    pub nickname: String,
    pub realname: String,
    pub username: String,
    pub hostname: String,
    pub servername: String,
}
