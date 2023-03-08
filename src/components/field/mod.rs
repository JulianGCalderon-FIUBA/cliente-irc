//! This modules defines the [Field] widget

mod imp;

use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;

glib::wrapper! {
    /// This widgets is used to ask for user for information, indicating the name of the variable.
    /// Input can be locked if value may not be changed.
    ///
    /// A default value can be set if the user does not input any value.
    ///
    /// If the user inputs an invalid value, an error message can be displayed
    ///
    /// Subclassifies [´gtk::Box`]
    pub struct Field(ObjectSubclass<imp::Field>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Field {
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Gets the user input, or default if no input was provided.
    pub fn input(&self) -> String {
        let input: String = self.property("input");

        if input.is_empty() {
            return self.property("default");
        }

        input
    }

    /// Sets the user input to ´value´.
    pub fn set_input(&self, value: &str) {
        self.set_property("input", value);
    }

    /// Locks the input, preventing the user from modifying its value
    /// If no input was provided by te user, then the default value is set.
    pub fn lock(&self) {
        let input: String = self.property("input");
        if input.is_empty() {
            let default: String = self.property("default");
            self.set_input(&default);
        }

        self.set_property("locked", true);
    }

    /// Sets an error message on the widget.
    pub fn set_error(&self, message: &str) {
        self.set_property("error", message);
    }

    /// Unsets the error message.
    pub fn unset_error(&self) {
        self.set_property("error", "");
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
