//! This modules defines the [Field] widget

mod constant;
mod imp;

use glib::Object;
use gtk::{glib, prelude::ObjectExt};

pub use constant::FieldProperty;

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
        let input: String = self.property(&FieldProperty::Input);

        if input.is_empty() {
            return self.property(&FieldProperty::Default);
        }

        input
    }

    /// Sets the user input to ´value´.
    pub fn set_input(&self, value: &str) {
        self.set_property(&FieldProperty::Input, value);
    }

    /// Locks the input, preventing the user from modifying its value
    /// If no input was provided by te user, then the default value is set.
    pub fn lock(&self) {
        let input: String = self.property(&FieldProperty::Input);
        if input.is_empty() {
            let default: String = self.property(&FieldProperty::Default);
            self.set_input(&default);
        }

        self.set_property(&FieldProperty::Locked, true);
    }

    /// Sets an error message on the widget.
    pub fn set_error(&self, message: &str) {
        self.set_property(&FieldProperty::Error, message);
    }

    /// Unsets the error message.
    pub fn unset_error(&self) {
        self.set_property(&FieldProperty::Error, "");
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
