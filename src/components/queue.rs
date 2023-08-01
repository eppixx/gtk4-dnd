use gtk::prelude::OrientableExt;
use relm4::{
    factory::FactoryVecDeque,
    gtk::{self},
    prelude::DynamicIndex,
    ComponentParts, ComponentSender, SimpleComponent,
};

use crate::factory::queue_item::QueueSong;

pub struct QueueModel {
    items: FactoryVecDeque<QueueSong>,
}

#[derive(Debug)]
pub enum QueueInput {
    MoveAbove {
        src: DynamicIndex,
        dest: DynamicIndex,
    },
    MoveBelow {
        src: DynamicIndex,
        dest: DynamicIndex,
    },
}

#[relm4::component(pub)]
impl SimpleComponent for QueueModel {
    type Input = QueueInput;
    type Output = ();
    type Init = ();

    fn init(
        _queue: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = QueueModel {
            items: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
        };

        //add test items
        model.items.guard().push_back(());
        model.items.guard().push_back(());
        model.items.guard().push_back(());
        model.items.guard().push_back(());

        let queue_items = model.items.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[local_ref]
            queue_items -> gtk::ListBox {
                set_selection_mode: gtk::SelectionMode::Multiple,
            },
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            QueueInput::MoveAbove { src, dest } => {
                let mut guard = self.items.guard();
                let src = src.current_index();
                let dest = dest.current_index();
                guard.move_to(src, dest);
            }
            QueueInput::MoveBelow { src, dest } => {
                let mut guard = self.items.guard();
                let src = src.current_index();
                let dest = dest.current_index();
                guard.move_to(src, dest + 1);
            }
        }
    }
}
