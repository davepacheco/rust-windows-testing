fn main() {
    let _matches = clap::App::new("demo")
        .arg(clap::Arg::with_name("foo").required(true))
        .get_matches();

    println!("{}\n", std::fs::read_to_string("help_file").unwrap());

    println!("{}", include_str!("file.txt"));
}
