use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    SimpleComponent,
};
use relm4_components::simple_combo_box::SimpleComboBox;

type ComboContent = &'static str;

const LANGS: &[ComboContent] = &[
    "English", "German", "French", "Polish", "Russian", "Chinese",
];
const GREETINGS: &[&str] = &["Hello!", "Hallo!", "Salut!", "Siema!", "привет!", "你好！"];

#[derive(Debug)]
enum AppMsg {
    ComboChanged(usize),
}

struct App {
    combo: Controller<SimpleComboBox<ComboContent>>,
    idx: usize,
}

impl App {
    fn lang(&self) -> &str {
        LANGS[self.idx]
    }

    fn greeting(&self) -> &str {
        GREETINGS[self.idx]
    }

    fn label(&self) -> String {
        format!("Greeting in {}: {}", self.lang(), self.greeting())
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_default_size: (300, 300),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                #[local_ref]
                combo -> gtk::ComboBoxText {},

                gtk::Label {
                    #[watch]
                    set_label: &model.label(),
                },
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            AppMsg::ComboChanged(idx) => self.idx = idx,
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let default_idx = 0;

        let combo = SimpleComboBox::builder()
            .launch(SimpleComboBox {
                variants: LANGS.to_vec(),
                active_index: Some(default_idx),
            })
            .forward(sender.input_sender(), |idx| AppMsg::ComboChanged(idx));

        let model = App {
            combo,
            idx: default_idx,
        };

        let combo = model.combo.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.combo_box");
    app.run::<App>(());
}
