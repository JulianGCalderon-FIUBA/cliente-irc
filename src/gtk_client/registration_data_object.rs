//! Defines the RegistrationDataObject struct

use client::data::RegistrationData;
use gtk::glib::{self, Object};

glib::wrapper! {
    /// A wrapper for the RegistrationData struct from the client crate
    ///
    /// Subclass of glib::Object to allow it to be used as a GObject
    ///
    /// # Properties
    ///
    /// * `nickname` - The nickname of the user
    ///     * type: `String`
    /// * `username` - The username of the user
    ///     * type: `String`
    /// * `realname` - The realname of the user
    ///     * type: `String`
    /// * `hostname` - The hostname of the user
    ///     * type: `String`
    /// * `servername` - The servername of the user
    ///     * type: `String`
    pub struct RegistrationDataObject(ObjectSubclass<imp::RegistrationDataObject>);
}

impl RegistrationDataObject {
    /// Creates a new RegistrationDataObject
    pub fn new(registration_data: RegistrationData) -> Self {
        Object::builder()
            .property("nickname", registration_data.nickname)
            .property("username", registration_data.username)
            .property("realname", registration_data.realname)
            .property("hostname", registration_data.hostname)
            .property("servername", registration_data.servername)
            .build()
    }
}

impl Default for RegistrationDataObject {
    fn default() -> Self {
        Self::new(RegistrationData::default())
    }
}

mod imp {
    use client::data::RegistrationData;
    use gtk::glib::once_cell::sync::Lazy;
    use gtk::glib::{self, ParamSpec, ParamSpecString};
    use gtk::prelude::ToValue;
    use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass};

    use std::cell::RefCell;

    #[derive(Default)]
    pub struct RegistrationDataObject {
        userdata: RefCell<RegistrationData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RegistrationDataObject {
        const NAME: &'static str = "UserData";
        type Type = super::RegistrationDataObject;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for RegistrationDataObject {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("nickname").build(),
                    ParamSpecString::builder("username").build(),
                    ParamSpecString::builder("realname").build(),
                    ParamSpecString::builder("hostname").build(),
                    ParamSpecString::builder("servername").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
            match pspec.name() {
                "nickname" => self.userdata.borrow_mut().nickname = value.get().unwrap(),
                "username" => self.userdata.borrow_mut().username = value.get().unwrap(),
                "realname" => self.userdata.borrow_mut().realname = value.get().unwrap(),
                "hostname" => self.userdata.borrow_mut().hostname = value.get().unwrap(),
                "servername" => self.userdata.borrow_mut().servername = value.get().unwrap(),
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "nickname" => self.userdata.borrow().nickname.to_value(),
                "username" => self.userdata.borrow().username.to_value(),
                "realname" => self.userdata.borrow().realname.to_value(),
                "hostname" => self.userdata.borrow().hostname.to_value(),
                "servername" => self.userdata.borrow().servername.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
