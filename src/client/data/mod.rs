//! This modules defines [`UserData`]

mod constant;
mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;

pub use constant::UserDataProperty;

glib::wrapper! {
    /// Used to store user data as properties
    ///
    /// Subclassifies [´glib::GObject´], therefore it can comunicate well with Gtk4 rust bindings.
    pub struct UserData(ObjectSubclass<imp::UserData>);
}

impl UserData {
    /// Creates a new [`UserData`] with the given values
    pub fn new(
        nickname: String,
        realname: String,
        username: String,
        hostname: String,
        servername: String,
    ) -> Self {
        Object::builder()
            .property(&UserDataProperty::Nickname, nickname)
            .property(&UserDataProperty::Realname, realname)
            .property(&UserDataProperty::Username, username)
            .property(&UserDataProperty::Hostname, hostname)
            .property(&UserDataProperty::Servername, servername)
            .build()
    }

    /// Shortcut to access `nickname` property
    pub fn nickname(&self) -> String {
        self.property(&UserDataProperty::Nickname)
    }
}

impl Default for UserData {
    fn default() -> Self {
        Object::builder().build()
    }
}
