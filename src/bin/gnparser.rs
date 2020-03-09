#[macro_use]
extern crate clap;

fn main() {
    use clap::App;
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("gnparser.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(ref input) = matches.value_of("INPUT") {
        println!("Using input file: {}", input);
    }
}

