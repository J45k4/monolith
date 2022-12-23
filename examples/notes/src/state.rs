
pub struct Note {
    pub id: String,    
    pub title: String,
    pub content: String,

}

pub struct State {
    next_id: u32,
    pub path: String,
    pub notes: Vec<Note>
}

impl State {
    pub fn new() -> State {
        State {
            next_id: 1,
            path: "/".to_string(),
            notes: vec![]
        }
    }

    pub fn get_note(&self, id: &str) -> Option<&Note> {
        self.notes.iter().find(|note| note.id == id)
    }

    pub fn create_new_note(&mut self) -> &mut Note {
        let note = Note {
            id: self.next_id.to_string(),
            title: "New note".to_string(),
            content: "".to_string()
        };

        self.next_id += 1;

        self.notes.push(note);

        self.notes.last_mut().unwrap()
    }
}
