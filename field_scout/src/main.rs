use std::{path::{Path, PathBuf}, time::Duration};

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

const IMAGE_EXTENSIONS: [&str; 4] = ["png", "jpg", "jpeg", "bmp"];

fn main() {
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 1 second
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    for result in rx {
        match result {
            Ok(events) => events.iter().for_each(|event| {
                if event.kind.is_create() {
                    event.paths.iter().for_each(|path| handle_created_path(path));
                }
            }),
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }
}

fn handle_created_path(path_buf: &PathBuf) {
    if is_an_image(path_buf) {
        send_file_to_objsto(path_buf)
    }
}

fn send_file_to_objsto(path_buf: &PathBuf) {
    println!("TODO: Sending {} to objsto", path_buf.to_str().unwrap_or_default());
    // TODO: Send to Object Storage here!
}

fn is_an_image(path: &PathBuf) -> bool {
    let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();
    return IMAGE_EXTENSIONS.contains(&extension)
}
