fn parse_markdown_file() {

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
    usage();
}
