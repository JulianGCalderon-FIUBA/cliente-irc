//! Pages are the main components of the application.
//!
//! They are advanced widgets that are used to build the user interface.
//! They are usually composed of multiple components.
//! They are also responsible for the application logic.

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
