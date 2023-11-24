use std::{cell::RefCell, rc::Rc};

use self::model::TextNoteModel;

pub mod ui;
pub mod model;


pub fn text_note_run() {
    let model = Rc::new(RefCell::new(TextNoteModel::new()));

    ui::run_textnote_app(model.clone());

    let model = model.borrow();

    println!("{}", model.text);
}