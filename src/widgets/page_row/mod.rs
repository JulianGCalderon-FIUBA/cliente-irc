/// This module defines [`PageRow`] related structures
mod constant;
mod imp;

use glib::Object;
use gtk::{
    glib::{self},
    prelude::ObjectExt,
    StackPage,
};

pub use constant::PageRowProperty;

glib::wrapper! {
    /// Used to display all sesion pages and switch between them
    ///
    /// If a page name in trailed with 'config', it is displayed in the config section
    /// If a page name is trailed with 'chat', it is displayed in the chat section
    ///
    /// If a page has an icon, it is displayed with the title
    ///
    /// Has a single css node 'sidebar'
    ///
    /// Subclassifies [`gtk::Box`]
    pub struct PageRow(ObjectSubclass<imp::PageRow>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl PageRow {
    /// Creates a new [´PageRow´] with associated [´StackPage´]
    pub fn new(page: StackPage) -> Self {
        let row = Object::builder().build();

        page.bind_property("title", &row, &PageRowProperty::Name)
            .sync_create()
            .transform_to(|_, title: Option<String>| title)
            .build();
        page.bind_property("icon-name", &row, &PageRowProperty::Icon)
            .sync_create()
            .transform_to(|_, icon: Option<String>| icon)
            .build();

        row
    }
}
