use odysee_dl::config::Config;
use odysee_dl::event::Event;
use odysee_dl::run;

static HELP: &str = "Usage: odysee-dl-cli [OPTIONS] <URL>

Download all content from an Odysee channel. <URL> should be the URL of the channel,
e.g. https://odysee.com/@channel:1

Options:
    -h, --help          Print help information
    -d, --dir <DIR>     Output directory (default: .)
    -r, --resume        Resume an interrupted download
";

fn main() {
    let config = parse_args().unwrap_or_else(|e| handle_error(e));
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        run(config, tx).unwrap_or_else(|e| handle_error(e));
    });

    for event in rx {
        render_event(&event);
    }

    handle.join().map_err(|e| {
        eprintln!("ERROR: {:?}", e);
        std::process::exit(1);
    }).ok();
}

enum ParseArgsErr {
    MissingUrl,
    MissingDirValue,
    UnknownOption(String),
}

impl std::fmt::Display for ParseArgsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseArgsErr::MissingUrl => write!(f, "Missing URL"),
            ParseArgsErr::MissingDirValue => write!(f, "Missing value for --dir"),
            ParseArgsErr::UnknownOption(option) => write!(f, "Unknown option: {}", option),
        }
    }
}

fn handle_error(e: impl std::fmt::Display) -> ! {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}

fn parse_args() -> Result<Config, ParseArgsErr> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(ParseArgsErr::MissingUrl);
    }

    let mut output_dir = std::path::PathBuf::from(".");
    let mut resume = false;
    let mut i = 1;

    while i < args.len() - 1 {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("{}", HELP);
                std::process::exit(0);
            }
            "-d" | "--dir" => {
                i += 1;
                if i >= args.len() - 1 {
                    return Err(ParseArgsErr::MissingDirValue);
                }
                output_dir = std::path::PathBuf::from(&args[i]);
            }
            "-r" | "--resume" => {
                resume = true;
            }
            _ => return Err(ParseArgsErr::UnknownOption(args[i].clone())),
        }
        i += 1;
    }

    let url = args.last().unwrap().to_string();
    Ok(Config { url, output_dir, resume })
}

fn render_event(event: &Event) {
    use std::io::Write;
    match event {
        Event::GetChannelStarted(url) => {
            print!("Fetching channel: {}...", url);
            std::io::stdout().flush().ok();
        }
        Event::GetChannelFailed(url, err) => eprintln!("\nFailed to fetch channel {}: {}", url, err),
        Event::GetChannelFinished(_) => println!(" Done"),
        Event::GetPostsStarted(url) => {
            print!("Fetching posts from: {}...", url);
            std::io::stdout().flush().ok();
        }
        Event::GetPostsFailed(url, err) => eprintln!("\nFailed to fetch posts from {}: {}", url, err),
        Event::GetPostsFinished(_) => println!(" Done"),
        Event::DownloadPostStarted(name) => {
            print!("Downloading: {}...", name);
            std::io::stdout().flush().ok();
        }
        Event::DownloadPostFailed(_, err) => println!(" Failed: {}", err),
        Event::DownloadPostSkipped(_) => println!(" Skipped"),
        Event::DownloadPostFinished(_) => println!(" Done"),
        Event::RateLimited(ctx) => println!("\nRate limited ({}), waiting 60s...", ctx),
        Event::Done => println!("Done."),
    }
}
