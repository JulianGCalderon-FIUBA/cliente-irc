use async_std::channel::{Receiver, Sender};
use gtk::glib;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::subclass::prelude::*;

use crate::message::{IrcCommand, IrcMessage};

/// IrcClient internal struct.
/// Fields wrapped in [OnceCell] are declared after construction
#[derive(Default)]
pub struct IrcClient {
    pub sender: OnceCell<Sender<IrcCommand>>,
    pub receiver: OnceCell<Receiver<IrcMessage>>,
}

#[glib::object_subclass]
impl ObjectSubclass for IrcClient {
    const NAME: &'static str = "IrcClient";
    type Type = super::IrcClient;
}

impl ObjectImpl for IrcClient {}
