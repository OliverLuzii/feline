mod flags;
use crate::flags::FlaggedString;
use std::{env, fs, io};

fn separate_args<T>(arg_iterator: T) -> (Vec<String>, Vec<String>)
where
    T: Iterator<Item = String>,
{
    let (flags, file_paths): (Vec<_>, Vec<_>) =
        arg_iterator.skip(1).partition(|x| x.starts_with("-"));

    if file_paths.len() == 0 {
        eprintln!("Missing file path, aborting.");
        std::process::exit(1);
    }

    (file_paths, flags)
}

fn read_or_abort(file_path: &str) -> String {
    let file_handle = match fs::read_to_string(file_path) {
        Ok(_file_handle) => _file_handle,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                eprintln!(
                    "File {} not found, aborting.",
                    file_path.split(&['\\', '/']).last().unwrap_or("")
                );
                std::process::exit(1);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied, aborting.");
                std::process::exit(1);
            }
            _ => {
                eprintln!("An unexpected error happened, aborting.");
                eprintln!("{:?}", error);
                std::process::exit(1);
            }
        },
    };

    file_handle
}

fn main() {
    let (file_paths, flags) = separate_args(env::args());
    let file_contents: Vec<_> = file_paths.into_iter().map(|x| read_or_abort(&x)).collect();

    let concatenated_files = file_contents.join("\n");
    let flagged_string = FlaggedString::new(concatenated_files, &flags);

    println!("{}", flagged_string.make_string());
}
