use async_std::channel::{Receiver, Sender};
use gtk::glib;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::subclass::prelude::*;

use crate::message::{IrcCommand, IrcMessage};

// Object holding the state
#[derive(Default)]
pub struct IrcClient {
    pub sender: OnceCell<Sender<IrcCommand>>,
    pub receiver: OnceCell<Receiver<IrcMessage>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for IrcClient {
    const NAME: &'static str = "IrcClient";
    type Type = super::IrcClient;
}

// Trait shared by all GObjects
impl ObjectImpl for IrcClient {}
