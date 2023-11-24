pub struct TextNoteModel {
    pub text: String,
}

impl TextNoteModel {

    pub fn new() -> Self {
        TextNoteModel { 
            text: String::from(""), 
        }
    }
    
}