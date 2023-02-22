use async_std::net::ToSocketAddrs;
use async_std::task::block_on;
use gtk::glib::{clone, closure_local, Object};
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::message::Message;
use crate::registration_window::RegistrationWindow;
use crate::server::Server;

const APPLICATION_ID: &str = "com.jgcalderon.irc-client";

mod imp {
    use gtk::glib::once_cell::sync::OnceCell;
    use gtk::glib::{self, WeakRef};
    // use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::traits::GtkWindowExt;

    use crate::registration_window::RegistrationWindow;
    use crate::server::Server;

    #[derive(Default)]
    pub struct Application {
        pub registration_window: WeakRef<RegistrationWindow>,
        pub server: OnceCell<Server>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for Application {}
    impl GtkApplicationImpl for Application {}
    impl ApplicationImpl for Application {
        fn activate(&self) {
            let application = self.obj();

            self.parent_activate();

            application.setup_registration();
            // application.handle_server();

            application.registration_window().present();
        }
    }
}

impl Application {
    fn registration_window(&self) -> RegistrationWindow {
        self.imp().registration_window.upgrade().unwrap()
    }

    fn server(&self) -> Server {
        self.imp().server.get().unwrap().clone()
    }
}

impl Application {
    fn setup_registration(&self) {
        let registration_window = RegistrationWindow::new(self);

        self.imp()
            .registration_window
            .set(Some(&registration_window));

        registration_window.connect_closure(
            "connect-button-clicked",
            true,
            closure_local!( @strong self as application =>
                move |_: RegistrationWindow| {
                    application.register_client()
                }
            ),
        );
    }

    fn handle_server<A: ToSocketAddrs>(&self, address: A) {
        let mut server = block_on(Server::connect(address)).unwrap();
        self.imp().server.set(server.clone()).unwrap();

        glib::MainContext::default().spawn_local(
            clone!(@strong self as application => async move {
                    while let Ok(message) = server.receive().await {
                        match Message::new(&message) {
                            Ok(message) => application.handle_message(message),
                            Err(error) => eprintln!("Error while parsing server message, {error:?}"),
                        }
                    }
            }),
        );
    }

    fn handle_message(&self, message: Message) {
        println!("{message}");
    }

    fn register_client(&self) {
        let registration_window = self.registration_window();

        let address: String = registration_window.property("address");
        self.handle_server(address);

        let password: String = registration_window.property("password");
        let nickname: String = registration_window.property("nickname");
        let username: String = registration_window.property("username");
        let realname: String = registration_window.property("realname");

        let pass_command = format!("PASS {password}");
        let nick_command = format!("NICK {nickname}");
        let user_command = format!("USER {username} :{realname}");

        block_on(async {
            self.server().send(pass_command).await.unwrap();
            self.server().send(nick_command).await.unwrap();
            self.server().send(user_command).await.unwrap();
        })
    }
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        Object::builder()
            .property("application-id", APPLICATION_ID)
            .build()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
