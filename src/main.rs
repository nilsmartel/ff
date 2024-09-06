use anyhow::Result;
use std::{sync::mpsc, thread};

fn print_help() -> ! {
    println!(
        "usage:
        ff <substring>
        
        Example: ff .rs"
    );

    std::process::exit(0);
}

fn main() {
    let file_to_find = std::env::args().nth(1).expect("one argument as filename");
    if file_to_find == "--help" || file_to_find == "-h" {
        print_help();
    }

    let (snd, rc) = mpsc::channel();

    thread::spawn(move || {
        find(".", &file_to_find, &snd).expect("read root directory");
    });

    for f in rc {
        println!("{f}");
    }
}

fn find(dir: &str, file_to_find: &str, result: &mpsc::Sender<String>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let Ok(entry) = entry else {
            continue;
        };

        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };

        let dirname = format!("{dir}/{}", &name);

        if entry.metadata().unwrap().is_dir() {
            let res = find(&dirname, file_to_find, result);
            if let Err(e) = res {
                eprintln!("error reading dir {}: {}", dir, e);
            }
            continue;
        }

        if name.contains(file_to_find) {
            result.send(dirname).expect("send to channel");
        }
    }

    Ok(())
}
