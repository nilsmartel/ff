use std::{sync::mpsc, thread};


fn main() {
    let file_to_find = std::env::args().nth(1).expect("one argument as filename");

    let (snd, rc) = mpsc::channel();

    thread::spawn(move || {
        find(".", &file_to_find, &snd);
    });

    for f in rc {
        println!("{f}");
    }
}

fn find(dir: &str, file_to_find: &str, result: &mpsc::Sender<String>) {

    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.expect("to unwrap dir entry");
        let name = entry.file_name().into_string().expect("valid string");

        let dirname = format!("{dir}/{}", &name);

        if entry.metadata().unwrap().is_dir() {

            find(&dirname, file_to_find, &result);
            continue;
        }

        if name.contains(file_to_find) {
            result.send(dirname).expect("to send to channel");
        }
    }
}
