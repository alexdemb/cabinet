use adw::{prelude::*, Application, ApplicationWindow};

pub struct AddTextDialogOutput {
    text: String,
    tags: Vec<String>
}

pub fn show_add_text_dialog() -> AddTextDialogOutput {

    let app = adw::Application::builder()
        .application_id("org.github.alexdemb.Cabinet.AddText")
        .build();

    app.connect_activate(build_ui);

    let exit_code = app.run();

    AddTextDialogOutput { text: String::from(""), tags: vec![String::from("")] }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .title("Add text")
        .build();

    window.present();
}