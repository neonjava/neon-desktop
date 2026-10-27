use std::{
    env,
    io::{self, Write},
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("apps") | Some("search") => {
            let query = args.next();
            if args.next().is_some() {
                eprintln!("unexpected argument");
                return ExitCode::FAILURE;
            }
            let stdout = io::stdout();
            let mut output = stdout.lock();
            let apps = neon_core::applications();
            for app in neon_core::search_applications(&apps, query.as_deref().unwrap_or("")) {
                if let Err(error) = writeln!(output, "{}\t{}", app.id, app.name) {
                    if error.kind() == io::ErrorKind::BrokenPipe {
                        return ExitCode::SUCCESS;
                    }
                    eprintln!("could not write applications: {error}");
                    return ExitCode::FAILURE;
                }
            }
            ExitCode::SUCCESS
        }
        Some("launch") => {
            let Some(id) = args.next() else {
                eprintln!("usage: neon-session launch APP.desktop");
                return ExitCode::FAILURE;
            };
            if args.next().is_some() {
                eprintln!("unexpected argument");
                return ExitCode::FAILURE;
            }
            if !neon_core::applications().iter().any(|app| app.id == id) {
                eprintln!("application not found: {id}");
                return ExitCode::FAILURE;
            }
            match Command::new("gtk-launch").arg(&id).spawn() {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("could not launch {id}: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!("usage: neon-session apps | search QUERY | launch APP.desktop");
            ExitCode::FAILURE
        }
    }
}
