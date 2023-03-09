//! Defines the [`Field`] widget

mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::traits::WidgetExt;
use gtk::{glib, Entry, Label};

glib::wrapper! {
    /// The `Field` component is a widget that allows the user to input a value.
    ///
    /// Subclassifies [`gtk::Box`].
    ///
    /// # Features
    ///
    /// * The name of the field is shown as a label.
    ///
    /// * The input can be locked, preventing the user from changing it.
    ///
    /// * The default value is shown as a placeholder.
    ///
    /// * An error message can be displayed below the input.
    ///
    /// # Properties
    ///
    /// * `name`: The name of the field.
    /// * `default`: The default value of the field.
    /// * `input`: The user input.
    /// * `locked`: Whether the input is locked.
    /// * `error`: The error message. When empty, no error message is shown.
    ///
    /// # CSS nodes
    ///
    /// `Field` has a single CSS node with name `field`.
    ///
    ///
    pub struct Field(ObjectSubclass<imp::Field>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Field {
    /// Creates a new `Field` widget.
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Returns the user input.
    ///
    /// If the user has not provided any input, the default value is returned.
    pub fn input(&self) -> String {
        let input: String = self.property("input");

        if input.is_empty() {
            return self.property("default");
        }

        input
    }

    /// Sets the user input.
    pub fn set_input(&self, value: &str) {
        self.set_property("input", value);
    }

    /// Locks the input.
    /// If the user has not provided any input, the default value is set.
    pub fn lock(&self) {
        let input: String = self.property("input");
        if input.is_empty() {
            let default: String = self.property("default");
            self.set_input(&default);
        }

        self.set_property("locked", true);
    }

    /// Sets the error message.
    pub fn set_error(&self, message: &str) {
        self.set_property("error", message);
    }

    /// Unsets the error message.
    pub fn unset_error(&self) {
        self.set_property("error", "");
    }

    /// Setups the object bindings.
    fn setup_bindings(&self) {
        self.bind_error_visibility();

        self.bind_locked_icon_visibility();

        self.bind_invalid_css_class();
    }

    /// Binds the error visibility to the error property.
    fn bind_error_visibility(&self) {
        self.bind_property::<Label>("error", &self.imp().error_label, "visible")
            .transform_to(|_, error: String| Some(!error.is_empty()))
            .build();
    }

    /// Binds the invalid CSS class to the error property.
    fn bind_invalid_css_class(&self) {
        self.connect_notify(Some("error"), |field, _| {
            let error: String = field.property("error");
            if error.is_empty() {
                field.remove_css_class("invalid");
            } else {
                field.add_css_class("invalid");
            };
        });
    }

    /// Binds the locked icon visibility to the locked property.
    fn bind_locked_icon_visibility(&self) {
        self.bind_property::<Entry>("locked", &self.imp().entry, "secondary-icon-name")
            .transform_to(|_, locked: bool| {
                if locked {
                    Some("system-lock-screen-symbolic")
                } else {
                    Some("")
                }
            })
            .build();
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
