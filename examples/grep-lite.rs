fn main() {
    let search_term = "picture";
    let quotes = "\
Every face, every shop,
dark square picture
It is the same with books.";

    let mut line_number: usize = 1;
    for line in quotes.lines() {
        if line.contains(search_term) {
            println!("{}: {}",line_number, line);
        }
        line_number += 1;
    }
}
