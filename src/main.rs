fn main() {
    clap::App::new("demo")
        .arg(clap::Arg::with_name("foo").required(true))
        .get_matches();
}
