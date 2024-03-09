pub mod admins;

use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton};

const SPECIAL_CHARS: &[char] = &[
    '\\', '_', '*', '[', ']', '(', ')', '~', '`', '>', '<', '&', '#', '+', '-', '=', '|', '{', '}', '.', '!',
];

pub fn make_help_keyboard(datas: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard_buttons: Vec<Vec<InlineKeyboardButton>> = vec![];

    for data in datas.chunks(3) {
        let row = data
            .iter()
            .map(|&d| InlineKeyboardButton::callback(pascal_case(d), "help_".to_owned() + d))
            .collect();
        keyboard_buttons.push(row);
    }

    InlineKeyboardMarkup::new(keyboard_buttons)
}

pub fn pascal_case(s: &str) -> String {
    let mut pascal_case = String::new();
    let mut capitalize_next = true; // Start with capitalizing the first character

    for c in s.chars() {
        if c == '_' || c == ' ' || c == '-' {
            capitalize_next = true; // Capitalize the next character after encountering a separator
        } else {
            if capitalize_next {
                pascal_case.push(c.to_ascii_uppercase()); // Capitalize the current character
                capitalize_next = false;
            } else {
                pascal_case.push(c);
            }
        }
    }

    pascal_case
}

pub fn escape_markdown(data: &str) -> String {
    let mut return_data = String::new();
    for c in data.chars() {
        if SPECIAL_CHARS.contains(&c) {
            return_data.push('\\');
        }
        return_data.push(c)
    }

    return return_data;
}
