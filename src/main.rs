use structopt::StructOpt;

/// search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// the pattern to look for
    #[structopt(short = "p", long = "pattern")]
    pattern: String,
    /// the path to the file to read
    #[structopt(short = "o", long = "path")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    // println!("search for {:#?} in {:?}", args.pattern, args.path)
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
