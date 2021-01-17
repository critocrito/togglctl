use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs::{self, create_dir_all};
use std::path::PathBuf;

const HELP: &str = "\
USAGE: togglctl [-dhV] command [command_args]

FLAGS:
  -d, --debug           Enable debug output
  -h, --help            Prints help information
  -V, --version         Print version information

COMMANDS:
  set-auth              Cache the Toggl API token
  projects              List all projects
  start <project>       Start a timer for <project>
  stop                  Stop the currently running timer
";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Args {
    debug: bool,
}

fn print_help() {
    print!("{}", HELP);
    std::process::exit(0);
}

fn print_version() {
    print!("{}", VERSION);
    std::process::exit(0);
}

fn abort(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn token_cache() -> Result<PathBuf> {
    let project = ProjectDirs::from("net", "smoking-heaps", "togglctl").unwrap();
    let cfg_dir = project.config_dir();
    create_dir_all(&cfg_dir)
        .with_context(|| format!("Failed to create config dir {:?}", cfg_dir))?;
    Ok(cfg_dir.join("api_token"))
}

fn load_token() -> Result<String> {
    let token_path = token_cache()?;
    let token = fs::read_to_string(token_path)
        .context("Failed to load API token. Maybe try: togglctl set-auth <token>")?;
    Ok(token)
}

fn store_token(token: &str) -> Result<()> {
    let token_path = token_cache()?;
    fs::write(token_path, token)?;
    Ok(())
}

fn main() {
    let mut pargs = pico_args::Arguments::from_env();

    // Handle a call for help right away.
    if pargs.contains(["-h", "--help"]) {
        print_help();
    }

    // The next priority is to print the version.
    if pargs.contains(["-V", "--version"]) {
        print_version();
    }

    let args = Args {
        debug: pargs.contains(["-d", "--debug"]),
    };

    let subcommand = match pargs.subcommand().unwrap() {
        None => return print_help(),
        Some(s) => s,
    };

    match subcommand.as_str() {
        "set-auth" => {
            let token = match pargs.subcommand().unwrap() {
                None => return print_help(),
                Some(s) => s,
            };
            if let Err(e) = store_token(&token) {
                return abort(&e.to_string());
            }
        }
        "projects" => {
            let project = match pargs.subcommand().unwrap() {
                None => return print_help(),
                Some(s) => s,
            };
            let token = match load_token() {
                Ok(t) => t,
                Err(e) => {
                    return abort(&e.to_string());
                }
            };

            print!("{} {}, {:?}, {}", "projects", project, args, token);
        }
        "start" => {
            let token = match load_token() {
                Ok(t) => t,
                Err(e) => {
                    return abort(&e.to_string());
                }
            };

            print!("{}, {:?}, {}", "start", args, token);
        }
        "stop" => {
            let token = match load_token() {
                Ok(t) => t,
                Err(e) => {
                    return abort(&e.to_string());
                }
            };

            print!("{}, {:?}, {}", "stop", args, token);
        }
        _ => print_help(),
    };

    std::process::exit(0);
}
