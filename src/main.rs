fn main() {
    let _matches = clap::App::new("demo")
        .arg(clap::Arg::with_name("foo"))
        .get_matches();

    println!("{}", include_str!("file.txt"));
}
