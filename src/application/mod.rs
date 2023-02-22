mod imp;

mod chat;
mod main_window;
mod registration_window;

use async_std::net::ToSocketAddrs;
use async_std::task::block_on;
use gtk::glib::{clone, closure_local, Object};
use gtk::prelude::{ApplicationExt, ObjectExt};
use gtk::subclass::prelude::*;
use gtk::traits::GtkWindowExt;
use gtk::{gio, glib};

use crate::message::IrcMessage;
use crate::server::Server;

use main_window::MainWindow;
use registration_window::RegistrationWindow;

const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

const QUIT_MESSAGE: &str = "Disconnecting client";

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

// GETTERS
impl Application {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APPLICATION_ID)
            .build()
    }

    fn registration_window(&self) -> RegistrationWindow {
        self.imp().registration_window.upgrade().unwrap()
    }

    fn main_window(&self) -> MainWindow {
        self.imp().main_window.upgrade().unwrap()
    }

    fn server(&self) -> Server {
        self.imp().server.get().unwrap().clone()
    }

    fn is_connected(&self) -> bool {
        self.imp().server.get().is_some()
    }
}

// SETUP
impl Application {
    fn setup_registration(&self) {
        let registration_window = RegistrationWindow::new(self);
        self.imp()
            .registration_window
            .set(Some(&registration_window));

        self.registration_window().connect_closure(
            "connect-button-clicked",
            true,
            closure_local!( @strong self as application =>
                move |_: RegistrationWindow| {
                    application.register_client()
                }
            ),
        );
    }

    fn setup_main_window(&self) {
        let main_window = MainWindow::new(self);

        self.imp().main_window.set(Some(&main_window));
    }

    fn handle_server<A: ToSocketAddrs>(&self, address: A) {
        let Ok(server) = block_on(Server::connect(address)) else { return };

        self.imp().server.set(server).unwrap();

        glib::MainContext::default().spawn_local(
            clone!(@strong self as application => async move {
                    while let Ok(message) = application.server().receive().await {
                        match IrcMessage::new(&message) {
                            Ok(message) => application.handle_message(message),
                            Err(error) => eprintln!("Error while parsing server message, {error:?}"),
                        }
                    }
            }),
        );
    }
}

// MESSAGE HANDLER
impl Application {
    fn handle_message(&self, message: IrcMessage) {
        match message {
            IrcMessage::Welcome { .. } => self.handle_welcome(),
            IrcMessage::Quit { .. } => self.handle_quit(),
        }
    }

    fn handle_welcome(&self) {
        self.setup_main_window();

        self.registration_window().close();
        self.main_window().present();
    }

    fn handle_quit(&self) {
        self.quit();
    }

    fn send_quit(&self) {
        let quit_message = IrcMessage::Quit {
            message: String::from(QUIT_MESSAGE),
        };

        block_on(self.server().send(quit_message)).unwrap();
    }
}

// LOGIC
impl Application {
    fn register_client(&self) {
        let registration_window = self.registration_window();

        if !self.is_connected() {
            let address: String = registration_window.property("address");
            self.handle_server(address);
        }

        if self.is_connected() {
            let password: String = registration_window.property("password");
            let nickname: String = registration_window.property("nickname");
            let username: String = registration_window.property("username");
            let realname: String = registration_window.property("realname");

            let pass_command = format!("PASS {password}");
            let nick_command = format!("NICK {nickname}");
            let user_command = format!("USER {username} :{realname}");

            glib::MainContext::default().spawn_local(
                clone!(@strong self as application => async move {
                    application.server().send(pass_command).await.unwrap();
                    application.server().send(nick_command).await.unwrap();
                    application.server().send(user_command).await.unwrap();
                }),
            );
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
