//! Defines the [`Message`] widget
mod imp;

use glib::Object;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, Label};

glib::wrapper! {
    /// The `Message` widget that displays a message.
    ///
    /// Subclassifies [`gtk::Box`].
    ///
    /// # Features
    ///
    /// * The sender of the message may be shown.
    ///
    /// # Properties
    ///
    /// * `message`: The message to be displayed.
    ///     * type: `String`
    /// * `sender`: The sender of the message.
    ///     When empty, the sender is not shown.
    ///     * type: `String`
    pub struct Message(ObjectSubclass<imp::Message>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Message {
    /// Creates a new `Message` widget.
    pub fn new(message: String) -> Self {
        Object::builder().property("message", message).build()
    }

    /// Sets the message sender.
    pub fn set_sender(&self, sender: String) {
        self.set_property("sender", sender);
    }

    /// Setups the object bindings.
    fn setup_bindings(&self) {
        self.bind_sender_visibility();
    }

    /// Binds the sender label visibility.
    fn bind_sender_visibility(&self) {
        let sender_label = &self.imp().sender_label;
        self.bind_property::<Label>("sender", sender_label, "visible")
            .transform_to(|_, sender: String| Some(!sender.is_empty()))
            .sync_create()
            .build();
    }
}
