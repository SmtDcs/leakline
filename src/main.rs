use leakline::{inspect_hex, Policy, Status};
use std::env;
use std::io::{self, Read};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("leakline: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<u8, String> {
    let mut args = env::args().skip(1);
    let mut policy = Policy::FullPrivacy;
    let mut path: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--policy" => {
                let name = args.next().ok_or("--policy needs a zcashd policy name")?;
                policy = Policy::parse(&name).ok_or_else(|| format!("unknown policy {name}"))?;
            }
            "--version" | "-V" => {
                println!("leakline {}", env!("CARGO_PKG_VERSION"));
                return Ok(0);
            }
            "--help" | "-h" => {
                println!(
                    "\
leakline [--policy NAME] [file]
leakline --version
Reads hex from a file, or from stdin when no file is given. Prints one JSON line.
Exit 0 CLEAN, 1 LEAK, 2 usage or unreadable input.
NAME is a zcashd policy. The default is FullPrivacy.
FullPrivacy AllowRevealedAmounts AllowRevealedRecipients AllowRevealedSenders
AllowFullyTransparent AllowLinkingAccountAddresses NoPrivacy"
                );
                return Ok(0);
            }
            other if other.starts_with('-') => return Err(format!("unknown argument {other}")),
            other => {
                if path.is_some() {
                    return Err("pass one file, or no file to read stdin".into());
                }
                path = Some(other.to_string());
            }
        }
    }

    let text = match path {
        Some(path) => std::fs::read_to_string(&path).map_err(|err| format!("{path}: {err}"))?,
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|err| err.to_string())?;
            buf
        }
    };
    let verdict = inspect_hex(&text, policy)?;
    println!("{}", verdict.to_json());
    Ok(match verdict.status {
        Status::Clean => 0,
        Status::Leak => 1,
    })
}
