fn usage() {
    let the_version = get_version();
    println!("tinymd, a markdown compiler written by tlylt");
    println!("version: {}", the_version);
}

fn get_version() -> u16 {
    1000
}

fn main() {
    usage();
}

