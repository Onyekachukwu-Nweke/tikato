mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.

    fn create_todo_ticket(title: String, description: String) -> crate::Ticket {
        crate::Ticket::new(title, description, "To-Do".into())
    }
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
        if title.is_empty() {
            panic!("Title is empty");
        }

        if title.len() > 50 {
            panic!("Title is too long");
        }

        if description.is_empty() {
            panic!("Description is empty");
        }

        if description.len() > 500 {
            panic!("Description is too long");
        }
        
        match status.as_str() {
            "To-Do" | "In Progress" | "Done" => {},
            _ => panic!("Status is invalid"),
        }
        Self {
            title,
            description,
            status
        }
    }

    fn is_open(&self) -> bool {
        self.status == "Open"
    }
}

fn main() {
    let ticket = Ticket {
        title: "Build a ticket system".into(),
        description: "A kanban board".into(),
        status: "Open".into(),
    };

    let is_open = Ticket::is_open(&ticket);
    println!("Hello, world!");
    println!("Title: {}, Description: {}, {}", ticket.title, ticket.description, is_open);
}
