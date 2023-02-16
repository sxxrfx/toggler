
use std::process::Command;

use sysinfo::{ProcessExt, System, SystemExt};

use clap::Parser;

enum ProgramState {
    Running,
    NotRunning,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // name of the program to toggle
    program_name: String,

    // arguments to the program
    args: Vec<String>,
}

fn main() {
    let mut state = ProgramState::NotRunning;

    let s = System::new_all();

    let cli = Cli::parse();

    for a in cli.args.iter() {
        println!("{}", a);
    }
    //

    if s.processes_by_exact_name(&cli.program_name)
        .peekable()
        .peek()
        .is_some()
    {
        state = ProgramState::Running;
    };

    match state {
        ProgramState::Running => {
            for process in s.processes_by_exact_name(&cli.program_name) {
                process.kill();
            }
        }
        ProgramState::NotRunning => {
            Command::new(&cli.program_name)
                .args(cli.args.into_iter())
                .spawn()
                .expect("Error running the program.");
        }
    }

}
