//! Pages are the main components of the application.
//!
//! Advanced widgets that are used to build the user interface.
//! Usually composed of multiple components.
//! Responsible for the application logic.

mod account;
mod chat;
mod chat_adder;
mod login;
mod session;

pub use account::Account;
pub use chat::Chat;
pub use chat_adder::ChatAdder;
pub use login::Login;
pub use session::Session;
