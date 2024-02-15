mod case_sensitive_sort;
mod common;
mod median;
mod text_type;
mod unique_item;

use case_sensitive_sort::sort_usernames;
use median::find_median;
use text_type::print_text;
use unique_item::find_unique_item;

fn main() {
    println!("Starting median");
    find_median();
    find_unique_item(vec![1, 2, 2, 10, 6, 2, 11, 6, 10]);
    println!("{}", print_text(&String::from("Helllo")));
    println!("{}", print_text("Hello"));
}
