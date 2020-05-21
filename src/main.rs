fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[INFO ] Trying to parse {}...", _filename);
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut written_by = String::from("Written by: ");
    written_by.push_str(env!("CARGO_PKG_AUTHORS"));
    let mut homepage = String::from("Homepage: ");
    homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));
    let usage = String::from("Usage: tinymd <somefile>.md");
    println!("{}", written_by);
    println!("{}", homepage);
    println!("{}", usage);
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(&String::from(env!("CARGO_PKG_DESCRIPTION")));

    the_title
}

fn usage() {
    print_long_banner();
}

fn main() {
    // A vector in Rust is a type denoted by the keyword Vec, followed by < and >, 
    // with the variable type enclosed in the brackets.
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ERROR ] Invalid invocation!)");
            usage()
        }
    }
}
