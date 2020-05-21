fn usage() {
    let the_version = get_version();
    /*
    let the_version: u16;   // Declare our variable
    the_version = get_version();    // Assign a value to our variable
    */
    println!("tinymd, a markdown compiler written by Wony.");
    println!("Version {}", the_version);  // Print the value assigned
}

fn main() {
    usage();
    get_version();
}

fn get_version() ->u16 {
    1000
}