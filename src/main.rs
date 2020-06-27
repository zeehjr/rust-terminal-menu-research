mod term;

fn main() {
    let options: Vec<String> = vec![
        String::from("Option 1"),
        String::from("Option 2"),
        String::from("Option 3"),
        String::from("Option 4"),
        String::from("Option 5"),
        String::from("Option 6"),
    ];

    let response = 
        term::menu::run(
            &String::from("Please, select your choice?"), 
            &options, 
            &term::menu::SelectionType::Simple
        );

    if response.is_some() {
        println!("");
        println!("Responses:");
        for (index, text) in &response.unwrap().selected_items {
            println!("{index} - {text}", index=index, text=text);
        }
    } else {
        println!("\r\nexited")
    }
}