use relm4::{
    gtk::{
        self, gdk, glib, pango,
        prelude::ToValue,
        traits::{BoxExt, ButtonExt, ListBoxRowExt, OrientableExt, WidgetExt},
    },
    prelude::{DynamicIndex, FactoryComponent},
    FactorySender,
};

use crate::components::queue::QueueInput;

#[derive(Debug)]
pub struct QueueSong {
    root_widget: gtk::Box,
    drag_src: gtk::DragSource,
}

#[derive(Debug)]
pub enum QueueSongInput {
    DraggedOver(f64),
    DragDropped {
        src: DynamicIndex,
        dest: DynamicIndex,
        y: f64,
    },
    DragLeave,
}

#[derive(Debug)]
pub enum QueueSongOutput {
    MoveAbove {
        src: DynamicIndex,
        dest: DynamicIndex,
    },
    MoveBelow {
        src: DynamicIndex,
        dest: DynamicIndex,
    },
}

#[relm4::factory(pub)]
impl FactoryComponent for QueueSong {
    type ParentWidget = gtk::ListBox;
    type ParentInput = QueueInput;
    type CommandOutput = ();
    type Input = QueueSongInput;
    type Output = QueueSongOutput;
    type Init = ();
    type Widgets = QueueSongWidgets;

    fn init_model(_init: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let model = Self {
            root_widget: gtk::Box::new(gtk::Orientation::Vertical, 0),
            drag_src: gtk::DragSource::new(),
        };

        // prepare dragging source
        // let boxed = glib::BoxedAnyObject::new(index.clone());
        // let content = gdk::ContentProvider::for_value(&boxed.to_value());
        // model.drag_src.set_actions(gdk::DragAction::MOVE);
        // model.drag_src.set_content(Some(&content));
        // dbg!(&boxed.borrow::<DynamicIndex>());

        // test with String
        let content = gdk::ContentProvider::for_value(&String::from("test").to_value());
        model.drag_src.set_actions(gdk::DragAction::MOVE);
        model.drag_src.set_content(Some(&content));

        model
    }

    fn output_to_parent_input(output: Self::Output) -> Option<QueueInput> {
        match output {
            QueueSongOutput::MoveAbove { src, dest } => Some(QueueInput::MoveAbove { src, dest }),
            QueueSongOutput::MoveBelow { src, dest } => Some(QueueInput::MoveBelow { src, dest }),
        }
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            QueueSongInput::DraggedOver(y) => {
                self.root_widget.remove_css_class("padd-item");
                let height = self.root_widget.height();
                if y < height as f64 * 0.5f64 {
                    //in the top half of the widget
                    self.root_widget.add_css_class("drag-indicator-top");
                    self.root_widget.remove_css_class("drag-indicator-bottom")
                } else {
                    //in the bottom half of the widget
                    self.root_widget.add_css_class("drag-indicator-bottom");
                    self.root_widget.remove_css_class("drag-indicator-top")
                }
            }
            QueueSongInput::DragDropped { src, dest, y } => {
                let height = self.root_widget.height();
                if y < height as f64 * 0.5f64 {
                    sender.output(QueueSongOutput::MoveAbove { src, dest });
                } else {
                    sender.output(QueueSongOutput::MoveBelow { src, dest });
                }
            }
            QueueSongInput::DragLeave => {
                self.root_widget.remove_css_class("drag-indicator-top");
                self.root_widget.remove_css_class("drag-indicator-bottom");
                self.root_widget.add_css_class("padd-item");
            }
        }
    }

    view! {
        #[root]
        gtk::ListBoxRow {
            #[wrap(Some)]
            set_child = &self.root_widget.clone() {
                add_css_class: "queue-song",
                add_css_class: "padd-item",

                add_controller = &gtk::DropTarget {
                    set_actions: gdk::DragAction::MOVE,
                    // set_types: &[//glib::Type::INVALID,
                    //              glib::Type::UNIT,
                    //              glib::Type::I8,
                    //              glib::Type::U8,
                    //              glib::Type::BOOL,
                    //              glib::Type::I32,
                    //              glib::Type::U32,
                    //              glib::Type::I_LONG,
                    //              glib::Type::U_LONG,
                    //              glib::Type::I64,
                    //              glib::Type::U64,
                    //              glib::Type::F32,
                    //              glib::Type::F64,
                    //              glib::Type::STRING,
                    //              glib::Type::POINTER,
                    //              glib::Type::VARIANT,
                    //              glib::Type::INTERFACE,
                    //              glib::Type::ENUM,
                    //              glib::Type::BOXED,
                    //              glib::Type::OBJECT,
                    //              glib::Type::PARAM_SPEC,
                    //              glib::Type::FLAGS],

                    connect_accept => |_, drop| {
                        dbg!(&drop.formats().to_str());
                        dbg!("connect_accept");

                        // TODO do checking of formats, for now accept all drops

                        true
                    },

                    connect_drop[sender, index] => move |_target, value, _x, y| {
                        dbg!("start connect drop {value:?}");

                        let any = value.get::<glib::BoxedAnyObject>();
                        dbg!(&any);
                        sender.input(QueueSongInput::DragDropped { src: any.unwrap().borrow::<DynamicIndex>().clone(), dest: index.clone(), y });

                        // let any = value.get::<String>();

                        // todo!("extract DynamicIndex");

                        true
                    },

                    connect_motion[sender] => move |_widget, _x, y| {
                        sender.input(QueueSongInput::DraggedOver(y));
                        // may need to return other value for drag in future
                        gdk::DragAction::MOVE
                    },

                    connect_leave => QueueSongInput::DragLeave,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,

                    gtk::Label {
                        #[watch]
                        set_label: "test",
                        set_hexpand: true,
                        set_halign: gtk::Align::Start,
                        set_ellipsize: pango::EllipsizeMode::End,
                    },

                    gtk::Button {
                        set_icon_name: "view-more-symbolic",
                        set_tooltip_text: Some("drag to reorder"),
                        add_controller: &self.drag_src,
                    }
                }
            }
        }
    }
}
