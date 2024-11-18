use tmgitty::git::GitStatus;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);

    let status = GitStatus::new(&args[1]).unwrap();
    println!("{}", status);
}
