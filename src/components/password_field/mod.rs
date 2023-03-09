//! Defines the [`PasswordField`] widget

mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, LevelBar};

glib::wrapper! {
    /// The `PasswordField` widget that allows the user to input a password.
    ///
    /// Subclassifies [`gtk::Box`].
    ///
    /// # Features
    ///
    /// * The name of the field is shown as a label.
    /// * Password is hidden by default, but may be shown by clicking the eye icon.
    /// * Password strength is shown as a level bar.
    ///
    /// # Properties
    ///
    /// * `input`: The user input.
    ///    * type: `String`
    /// * `name`: The name of the field.
    ///    * type: `String`
    pub struct PasswordField(ObjectSubclass<imp::PasswordField>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl PasswordField {
    /// Creates a new `PasswordField` widget.
    pub fn new() -> Self {
        Object::builder().build()
    }

    /// Returns the user input.
    pub fn input(&self) -> String {
        self.property("input")
    }

    /// Setups the object bindings.
    fn setup_bindings(&self) {
        self.binds_password_strength();
    }

    /// Binds the password strength to the level bar.
    fn binds_password_strength(&self) {
        self.imp()
            .entry
            .bind_property::<LevelBar>("text", &self.imp().level_bar, "value")
            .transform_to(|_, password: String| Some(password_strength(password)))
            .build();
    }
}

/// Returns the password strength.
///
/// The strength is a value between 0 and 3.
fn password_strength(password: String) -> f64 {
    match password.len() {
        0 => 0.0,
        1..=4 => 1.0,
        5..=9 => 2.0,
        _ => 3.0,
    }
}

impl Default for PasswordField {
    fn default() -> Self {
        Self::new()
    }
}
