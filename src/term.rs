pub mod menu {
  use termion::event::Key;
  use termion::input::TermRead;
  use termion::raw::IntoRawMode;
  use std::io::{Write, stdin, stdout};

  pub enum SelectionType {
    Simple,
    Radio,
    Check
  }

  pub struct ExecutionResult {
    pub selected_items: Vec<(usize, String)>
  }

  pub fn render_item(selection_type: &SelectionType, text: &String, selected: bool, highlighted: bool) -> String {
    let start_color = if highlighted { termion::color::Fg(termion::color::Blue).to_string() } else { String::from("") };
    let end_color = termion::color::Fg(termion::color::Reset).to_string();

    let icon = match selection_type {
      SelectionType::Simple => {
        "   "
      },
      SelectionType::Check => {
        if selected { "[*]" } else { "[ ]" }
      },
      SelectionType::Radio => {
        if selected { "(*)" } else { "( )" }
      }
    };
    
    format!("{start_color}{icon} {text}{end_color}", icon=icon, text=text, start_color=start_color, end_color=end_color)
  }

  fn is_selected(selected_items: &Vec<usize>, index: &usize) -> bool {
    selected_items.iter().any(|item| item == index)
  }

  fn select_item(selected_items: &Vec<usize>, selection_type: &SelectionType, index: &usize) -> Vec<usize> {
    match selection_type {
      SelectionType::Simple => {
        vec![index.clone()]
      },
      SelectionType::Check => {
        if is_selected(&selected_items, &index) {
          selected_items.iter().cloned().filter(|&item| item.ne(index)).clone().collect()
        } else {
          let mut new_list = selected_items.clone();
          new_list.push(index.clone());
          new_list
        }
      },
      SelectionType::Radio => {
        vec![index.clone()]
      }
    }
  }

  pub fn run(title: &String, options: &Vec<String>, selection_type: &SelectionType) -> Option<ExecutionResult> {
    if options.len() == 0 {
      return None;
    }

    let mut selected_items: Vec<usize> = vec![];
    let mut cursor_position: usize = 0;
    
    let mut stdout = stdout().into_raw_mode().unwrap();

    loop {
      let stdin = stdin();

      write!(stdout,
        "{clear}{move_cursor}{title}{hide_cursor}",
        clear=termion::clear::All,
        move_cursor=termion::cursor::Goto(1, 1),
        title=title,
        hide_cursor=termion::cursor::Hide
      ).unwrap();

      println!("\r\n");

      for (index, option) in options.iter().enumerate() {
        let text = render_item(&selection_type, &option, is_selected(&selected_items, &index), cursor_position == index);
  
        write!(stdout, "{}", text).unwrap();
        println!("\r");
      }

      for c in stdin.keys() {
        match c.unwrap() {
          Key::Up => {
            if cursor_position > 0 {
              cursor_position = cursor_position - 1;
            }
            break;
          },
          Key::Down => {
            if cursor_position < options.len() - 1 {
              cursor_position = cursor_position + 1
            }
            break;
          },
          Key::Char(' ') => {
            match selection_type {
              SelectionType::Simple => {},
              _ => {
                selected_items = select_item(&selected_items, &selection_type, &cursor_position);
                break;
              }
            }
          },
          Key::Char('\n') => {
            match selection_type {
              SelectionType::Simple => {
                selected_items = select_item(&selected_items, &selection_type, &cursor_position);
              },
              _ => {}
            }
            let items: Vec<(usize, String)> = selected_items.iter().map(|index| { (index.clone(), options[index.clone()].clone()) }).collect();
            let mut sorted = items.clone();
            sorted.sort_by(|a, b| a.0.cmp(&b.0));

            write!(stdout, "{}", termion::cursor::Show).unwrap();

            return Some(ExecutionResult {
              selected_items: sorted
            })
          },
          Key::Char('q') => {
            write!(stdout, "{}", termion::cursor::Show).unwrap();
            return None;
          }
          _ => {}
        }
      }
    }
  }
}

mod tests {

  
  mod render_item {
    use super::super::menu::{render_item, SelectionType};

    #[test]
    fn can_generate_simple_item() {
      let selection_type = SelectionType::Simple;
      let text = String::from("Hello, try selecting an item");
      let selected = true;
      let highlighted = false;
      let render_item_result = render_item(&selection_type, &text, selected, highlighted);
      assert_eq!(render_item_result, format!("    {text}\u{1b}[39m", text=text));
    }

    #[test]
    fn can_generate_highlighted_item() {
      let selection_type = SelectionType::Simple;
      let text = String::from("Hello, try selecting an item");
      let selected = true;
      let highlighted = true;
      let render_item_result = render_item(&selection_type, &text, selected, highlighted);
      assert_eq!(render_item_result, format!("\u{1b}[38;5;4m    {text}\u{1b}[39m", text=text));
    }

    #[test]
    fn can_generate_radio_item() {
      let selection_type = SelectionType::Radio;
      let text = String::from("Hello, try selecting an item");
      let selected = true;
      let highlighted = false;
      let render_item_result = render_item(&selection_type, &text, selected, highlighted);
      assert_eq!(render_item_result, format!("(*) {text}\u{1b}[39m", text=text));
    }

    #[test]
    fn can_generate_check_item() {
      let selection_type = SelectionType::Check;
      let text = String::from("Hello, try selecting an item");
      let selected = true;
      let highlighted = false;
      let render_item_result = render_item(&selection_type, &text, selected, highlighted);
      assert_eq!(render_item_result, format!("[*] {text}\u{1b}[39m", text=text));
    }
}

}