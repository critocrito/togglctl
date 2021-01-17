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
            print!("{}, {:?}", "set-auth", args);
        }
        "projects" => {
            let project = match pargs.subcommand().unwrap() {
                None => return print_help(),
                Some(s) => s,
            };

            print!("{} {}, {:?}", "projects", project, args);
        }
        "start" => {
            print!("{}, {:?}", "start", args);
        }
        "stop" => {
            print!("{}, {:?}", "stop", args);
        }
        _ => print_help(),
    };
}
