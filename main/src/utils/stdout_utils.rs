pub fn printc(text: &str, rgb_color: (i32, i32, i32)) {
    let (r, g, b) = rgb_color;
    let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
    println!("{}{}\x1b[0m", color_code, text);
}

pub fn print_solita() {
    use std::fs;
    let contents = fs::read_to_string("src/resources/solita.txt").unwrap_or(String::from("     Solita     "));
    println!(
        "\x1b[38;2;30;144;255m {} \x1b[0m",
        contents.replace("\n", "\n\x1b[38;2;30;144;255m")
    );
}
