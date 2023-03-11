//! Defines the [`ChatAdder`] page

mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    /// The chat adder page is used to add new chats
    ///
    /// Subclassifies `gtk::Box`
    ///
    /// # Signals
    ///
    /// * `add-chat` - Emitted when a new chat must be added
    ///
    ///     Arguments:
    ///     - `String` - The title of the new chat
    ///         * Type: `String`
    ///
    /// # CSS nodes
    ///
    /// `ChatAdder` has a single CSS node with name `chat-adder`.
    pub struct ChatAdder(ObjectSubclass<imp::ChatAdder>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl ChatAdder {
    /// Creates a new chat adder page
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for ChatAdder {
    fn default() -> Self {
        Self::new()
    }
}
