use std::{fs::File, io::Read};

fn main() {
    // &str because this string literal is directly pulled from
    // the binary itself and points to memory address in the binary
    // where the literal is stored.
    let hexcodes_path: &str = "hexcodes.txt";
    let maybe_file = File::open(hexcodes_path);
    match maybe_file {
        Ok(mut file) => {
            let mut hexcodes = String::new();
            file.read_to_string(&mut hexcodes);
            hexcodes_to_rgb(hexcodes.as_str())
        }
        Err(error) => println!("{}", error.to_string()),
    }
}

fn hexcodes_to_rgb(hexcodes: &str) {
    let mut start_window = 0;
    let hexcode_size = 6;
    let all_codes = hexcodes.len();

    while start_window + hexcode_size < all_codes {
        // first plus 1 for array index
        let end_window = start_window + hexcode_size + 1;
        let mut code = &hexcodes[start_window..end_window];
        // strip the "#" character
        code = &code[1..];

        let red = u8::from_str_radix(&code[0..2], 16).unwrap();
        let blue = u8::from_str_radix(&code[2..4], 16).unwrap();
        let green = u8::from_str_radix(&code[4..6], 16).unwrap();

        println!("rgb({} {} {})", red, blue, green);

        // second plus 1 because each hexcode is separated by a \n
        start_window = end_window + 1;
    }
}
