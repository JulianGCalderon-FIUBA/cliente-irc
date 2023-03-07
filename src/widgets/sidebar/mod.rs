/// This module defines [`Sidebar`] related structures
mod constant;
mod imp;

use glib::Object;
use gtk::{
    glib::{self, clone},
    prelude::{CastNone, ListModelExt, ObjectExt},
    subclass::prelude::ObjectSubclassIsExt,
    Stack, StackPage,
};

pub use constant::SidebarProperty;

use super::PageRow;

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
    pub struct Sidebar(ObjectSubclass<imp::Sidebar>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Accessible, gtk::Buildable,
        gtk::ConstraintTarget, gtk::Orientable;
}

impl Sidebar {
    /// Creates a new [´Sidebar´] with associated stack
    pub fn new(stack: Stack) -> Self {
        Object::builder()
            .property(&SidebarProperty::Stack, stack)
            .build()
    }

    fn with_each_added_page(&self, page: StackPage) {
        let row = PageRow::new(page.clone());

        if let Some(name) = page.name() {
            if name.starts_with("config") {
                return self.imp().configs.append(&row);
            }
        }

        self.imp().chats.append(&row);
    }

    fn setup_stack(&self) {
        let pages = self.property::<Stack>(&SidebarProperty::Stack).pages();
        self.imp().model.borrow_mut().replace(pages.clone());

        for page_number in 0..pages.n_items() {
            let page = pages.item(page_number).and_downcast().unwrap();
            self.with_each_added_page(page);
        }

        pages.connect_items_changed(
            clone!(@weak self as sidebar => move |model, position, _deleted, added| {
                let new_indexes = position..position + added;

                new_indexes
                    .map(|i| model.item(i))
                    .map(|object| object.and_downcast().unwrap())
                    .for_each(|page| sidebar.with_each_added_page(page))
            }),
        );
    }
}
