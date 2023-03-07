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
        let row = PageRow::new(page);

        self.imp().configs.append(&row);
    }

    fn setup_stack(&self) {
        let pages = self.property::<Stack>(&SidebarProperty::Stack).pages();
        for page_number in 0..pages.n_items() {
            let page = pages.item(page_number).and_downcast().unwrap();
            self.with_each_added_page(page);
        }

        pages.connect_items_changed(
            clone!(@weak self as sidebar => move |_model, position, added, deleted| {
                let _added_position = position..position + added;

                println!("position: {position}, added: {added}, deleted: {deleted}");

                // added_position
                //     .map(|i| model.item(i))
                //     .map(|widget| widget.and_downcast().unwrap())
                //     .for_each(|page: StackPage| sidebar.with_each_added_page(page));
            }),
        );
    }
}
