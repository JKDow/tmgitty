use tmgitty::git::fetch_and_status;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);

    let status = fetch_and_status(&args[1]).unwrap();

    println!("{}", status);
}
