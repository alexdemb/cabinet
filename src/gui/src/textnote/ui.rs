use std::{cell::RefCell, rc::Rc};

use adw::{Application, prelude::*, Window, HeaderBar, gtk::{TextView, Scrollable, ScrolledWindow}};

use super::model::TextNoteModel;

const APP_ID: &str = "org.github.alexdemb.Cabinet.TextNote";
const TITLE: &str = "Cabinet";

pub fn run_textnote_app(model: Rc<RefCell<TextNoteModel>>) {
    let app: Application = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |a| {
        let window = build_window(a);
        let header = build_header();
        let vbox = build_vbox();
        let text_view = build_text_view();
        let text_view_scroll = build_text_view_scroll(&text_view);

        let m = model.clone();
        text_view.buffer().connect_changed(move |buff| {
            let bounds = buff.bounds();
            let text = buff.text(&bounds.0, &bounds.1, false).to_string();
            m.borrow_mut().text = text;
        });

        vbox.append(&header);
        vbox.append(&text_view_scroll);
        
        window.set_content(Some(&vbox));
        window.show();        
    });

    app.run();
}

fn build_window(app: &Application) -> Window {
    Window::builder()
        .application(app)
        .width_request(800)
        .height_request(600)
        .title(TITLE)
        .build()
}

fn build_header() -> HeaderBar {
    HeaderBar::builder()
        .build()
}

fn build_vbox() -> adw::gtk::Box {
    adw::gtk::Box::builder()
        .orientation(adw::gtk::Orientation::Vertical)
        .spacing(6)
        .vexpand(true)
        .build()
}

fn build_text_view() -> TextView {
    TextView::builder()
        .wrap_mode(adw::gtk::WrapMode::Word)
        .left_margin(6)
        .top_margin(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .vexpand(true)
        .build()
}

fn build_text_view_scroll(text_view: &TextView) -> ScrolledWindow {
    ScrolledWindow::builder()
        .child(text_view)
        .build()
}