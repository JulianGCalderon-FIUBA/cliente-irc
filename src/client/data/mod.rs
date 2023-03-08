//! This modules defines `UserData`
mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;

glib::wrapper! {
    /// `UserData` is used to store user information.
    ///
    /// Subclassifies `glib::GObject`.
    ///
    /// #### Properties:
    /// * `nickname`: The nickname of the user.
    /// * `realname`: The real name of the user.
    /// * `username`: The username of the user.
    /// * `hostname`: The hostname of the user.
    /// * `servername`: The name of the server the user is connected to.
    pub struct UserData(ObjectSubclass<imp::UserData>);
}

impl UserData {
    /// creates a new `UserData` with the given properties
    pub fn new(
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> Self {
        Object::builder()
            .property("nickname", nickname)
            .property("realname", realname)
            .property("username", username)
            .property("hostname", hostname)
            .property("servername", servername)
            .build()
    }

    /// Returns the nickname of the user.
    pub fn nickname(&self) -> String {
        self.property("nickname")
    }
}

impl Default for UserData {
    fn default() -> Self {
        Object::builder().build()
    }
}
