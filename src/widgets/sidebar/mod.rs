/// This module defines [`Sidebar`] related structures
mod constant;
mod imp;

pub use constant::SidebarProperty;
use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::{Cast, ListModelExt, ObjectExt};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{
    BuilderListItemFactory, BuilderScope, CustomFilter, FilterListModel, SelectionModel,
    SingleSelection, Stack, StackPage, INVALID_LIST_POSITION,
};

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

    fn stack(&self) -> Stack {
        self.property(&SidebarProperty::Stack)
    }

    fn setup_stack(&self) {
        let stack: Stack = self.property(&SidebarProperty::Stack);
        let selection = stack.pages();

        self.setup_config_view(selection.clone());
        self.setup_chats_view(selection);
    }

    fn setup_config_view(&self, selection: SelectionModel) {
        let factory = create_factory();

        let filter = config_filter();

        let selection_model = create_filtered_selection_model(selection, filter);
        self.setup_config_selection(selection_model.clone());

        self.imp().config_list.set_factory(Some(&factory));
        self.imp().config_list.set_model(Some(&selection_model));

        selection_model.connect_selected_item_notify(
            clone!(@weak self as sidebar => move |model| {
                let Some(selected) = model.selected_item() else {return};
                let page: StackPage = selected.downcast().unwrap();

                sidebar.select_page(page);
                sidebar.chat_selection().set_selected(INVALID_LIST_POSITION);
            }),
        );
    }

    fn setup_config_selection(&self, selection_model: SingleSelection) {
        self.imp()
            .config_selection
            .borrow_mut()
            .replace(selection_model);
    }

    fn select_page(&self, page: StackPage) {
        let Some(name) = page.name() else {return};
        self.stack().set_visible_child_name(&name);
    }

    fn chat_selection(&self) -> SingleSelection {
        self.imp().chat_selection.borrow().clone().unwrap()
    }

    fn config_selection(&self) -> SingleSelection {
        self.imp().config_selection.borrow().clone().unwrap()
    }

    fn setup_chats_view(&self, selection: SelectionModel) {
        let factory = create_factory();

        let filter = chat_filter();

        let selection_model = create_filtered_selection_model(selection, filter);
        self.setup_chat_selection(selection_model.clone());

        self.imp().chat_list.set_factory(Some(&factory));
        self.imp().chat_list.set_model(Some(&selection_model));

        selection_model.connect_selected_item_notify(
            clone!(@weak self as sidebar => move |model| {
                let Some(selected) = model.selected_item() else {return};
                let page: StackPage = selected.downcast().unwrap();

                sidebar.select_page(page);
                sidebar.config_selection().set_selected(INVALID_LIST_POSITION);
            }),
        );

        selection_model.connect_items_changed(
            clone!(@weak self as sidebar => move |model, position, _, added,| {
                if added > 0 {
                    model.set_selected(position);
                }
            }),
        );
    }

    fn setup_chat_selection(&self, selection_model: SingleSelection) {
        self.imp()
            .chat_selection
            .borrow_mut()
            .replace(selection_model);
    }
}

fn create_filtered_selection_model(
    selection: SelectionModel,
    filter: CustomFilter,
) -> SingleSelection {
    let filter_model = FilterListModel::new(Some(selection), Some(filter));
    let selection = SingleSelection::new(Some(filter_model));
    selection.set_can_unselect(true);

    selection
}

fn config_filter() -> CustomFilter {
    CustomFilter::new(|page| {
        let page = page.downcast_ref::<StackPage>().unwrap();

        let Some(name) = page.name() else {return false};

        name.starts_with("config")
    })
}

fn chat_filter() -> CustomFilter {
    CustomFilter::new(|page| {
        let page = page.downcast_ref::<StackPage>().unwrap();

        let Some(name) = page.name() else {return false};

        name.starts_with("chat")
    })
}

fn create_factory() -> BuilderListItemFactory {
    // let factory = SignalListItemFactory::new();

    // factory.connect_setup(|_, list_item| {
    //     let task_row = Label::new(Some("hola"));
    //     list_item
    //         .downcast_ref::<ListItem>()
    //         .unwrap()
    //         .set_child(Some(&task_row));
    // });

    // factory.connect_bind(move |_, list_item| {
    //     let page = list_item
    //         .downcast_ref::<ListItem>()
    //         .unwrap()
    //         .item()
    //         .and_downcast::<StackPage>()
    //         .unwrap();

    //     let label = list_item
    //         .downcast_ref::<ListItem>()
    //         .unwrap()
    //         .child()
    //         .and_downcast::<Label>()
    //         .unwrap();

    //     page.bind_property("title", &label, "label")
    //         .sync_create()
    //         .build();
    // });

    // factory

    BuilderListItemFactory::from_resource(
        BuilderScope::NONE,
        "/com/jgcalderon/irc-client/ui/sidebar-row.ui",
    )
}
