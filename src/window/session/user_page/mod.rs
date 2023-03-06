mod constant;
mod imp;

use glib::Object;
use gtk::glib;

use crate::client::UserData;

pub use constant::UserPageProperty;

glib::wrapper! {
    pub struct UserPage(ObjectSubclass<imp::UserPage>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl UserPage {
    pub fn new(data: UserData) -> Self {
        Object::builder()
            .property(&UserPageProperty::Data, data)
            .build()
    }
}
