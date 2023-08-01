use components::queue::QueueModel;
use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};

mod components;
mod factory;

struct AppModel {
    queue: Controller<QueueModel>,
}

#[derive(Debug)]
enum AppInput {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;

    type Output = ();
    type Init = ();

    // Initialize the UI.
    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let queue: Controller<QueueModel> =
            QueueModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    _ => todo!(),
                });
        let model = AppModel { queue };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {}
    }

    view! {
        #[root]
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 500,
            set_default_height: 700,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                },

                gtk::Label {
                    set_label: "test",
                },

                append: model.queue.widget(),
            }
        }
    }
}

fn setup_css() {
    let data = "
.padd-item {
margin-top: 1px;
margin-bottom: 1px;
}
.drag-indicator-top {
border-top: 1px solid Gray;
margin-bottom: 1px;
}

.drag-indicator-bottom {
border-bottom: 1px solid Gray;
margin-top: 1px;
}
";

    relm4::set_global_css(data);
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    setup_css();
    app.run::<AppModel>(());
}
