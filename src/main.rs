use std::{io::stdin, vec};

#[derive(Debug, Default)]
struct Author {
    name: String,
    gender: String,
}

#[derive(Debug)]
struct Book {
    id: i32,
    title: String,
    author: Author,
}

trait IBook {
    fn new() -> BookController;
    fn display_books(&self);
    fn input_book_dialog(&mut self);
    fn list_available_books(&self);
}

struct BookController {
    book_list: Vec<Book>,
}

impl IBook for BookController {
    fn display_books(&self) {
        println!("{:?}", self.book_list);
    }

    fn new() -> BookController {
        Self { book_list: vec![] }
    }

    fn input_book_dialog(&mut self) {
        let book_id = read_int("Input Book Id: ".to_owned());
        let book_title = read_string("Input Book Title: ".to_owned());
        let book_author = read_string("Input Author Name: ".to_owned());
        let author_gender = read_string("Input Author Gender: ".to_owned());

        let book = Book {
            id: book_id,
            title: book_title,
            author: Author {
                name: book_author,
                gender: author_gender,
            },
        };
        self.book_list.append(&mut vec![book]);
    }

    fn list_available_books(&self) {
        todo!()
    }
}

struct Menu {
    pub user_choice: i32,
}

impl Menu {
    pub fn print_menu(&self) {
        println!("===== Welcome =====");
        println!("[1]. Enter Book");
        println!("[2]. Display All Books");
        println!("[3]. List Available Books");
        println!("[4]. Borrow Book");
        println!("[5]. Return Book");
        println!("[0]. Exit");
        println!("Press (1-5 or 0): ")
    }

    pub fn get_user_input(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("not a valid input");
        self.user_choice = input.trim().parse::<i32>().expect("not a valid integer")
    }
}

fn read_string(input_message: String) -> String {
    println!("{}", input_message);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("not a valid input");
    input.trim().to_owned()
}

fn read_int(input_message: String) -> i32 {
    println!("{}", input_message);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("not a valid input");
    input.trim().parse::<i32>().expect("not a valid integer")
}

fn main() {
    let mut menu = Menu { user_choice: -1 };

    let mut book_controller = BookController::new();

    while menu.user_choice != 0 {
        menu.print_menu();
        menu.get_user_input();

        match menu.user_choice {
            1 => book_controller.input_book_dialog(),
            2 => book_controller.display_books(),
            3 => book_controller.list_available_books(),
            0 => println!("Exiting the program"),
            _ => println!("invalid input"),
        }
        println!("")
    }
}
