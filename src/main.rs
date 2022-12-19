use std::io::BufRead;
use std::path::PathBuf;

fn follow_symlink(path: PathBuf, include_hidden: bool) -> u128 {
    match path.to_str() {
        Some(path) => {
            match std::fs::read_link(path) {
                Ok(link_val) => {
                    match std::fs::metadata(link_val.clone()) {
                        Ok(rec_meta) => {
                            match link_val.clone().file_name() {
                                Some(file_name) => {
                                    match file_name.to_str() {
                                        Some(file_name) => {
                                            if file_name.starts_with(".") && !include_hidden {
                                                0u128
                                            } else if rec_meta.is_symlink() {
                                                follow_symlink(link_val, include_hidden)
                                            } else if rec_meta.is_dir() {
                                                match link_val.to_str() {
                                                    Some(link) => read_dir(link, include_hidden),
                                                    None => 0u128
                                                }

                                            } else if rec_meta.is_file() {
                                                read_file(link_val)
                                            } else {
                                                0u128
                                            }
                                        },
                                        None => {
                                            eprintln!("Could not transform os string into rust string!");
                                            0u128
                                        }
                                    }
                                },
                                None => {
                                    eprintln!("Could not retrieve filename from link");
                                    0u128
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                            0u128
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Could not read link '{}': {}", path, e);
                    0u128
                }
            }
        },
        None => {
            eprintln!("Could not convert path to string!");
            0u128
        }
    }
}

fn read_file(path: PathBuf) -> u128 {
    match std::fs::File::open(path.clone()) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let mut lines = 0u128;
            for line in reader.lines() {
                match line {
                    Ok(_) => lines += 1u128,
                    Err(_) => {
                        return 0u128;
                    }
                }
            }
            lines
        }
        Err(e) => {
            match path.to_str() {
                Some(path_str) => {
                    eprintln!("Could not open file {} for reading with error: {}", path_str, e);
                },
                None => {
                    eprintln!("Could not open file for reading with error: {}", e);
                }
            }
            0u128
        }
    }
}

fn read_dir(path: &str, include_hidden: bool) -> u128 {
    match std::fs::read_dir(path) {
        Ok(res) => {
            match res
                .map(|entry| match entry {
                    Ok(ent) => match ent.file_type() {
                        Ok(typ) => {
                            let abort = match ent.file_name().to_str() {
                                Some(file_name) => (file_name.starts_with(".") && include_hidden) || !file_name.starts_with("."),
                                None => true
                            };
                            if abort {
                                return 0u128;
                            }

                            if typ.is_dir() {
                                match ent.path().to_str() {
                                    Some(path) => {
                                        read_dir(path, include_hidden)
                                    },
                                    None => 0u128
                                }
                            } else if typ.is_file() {
                                read_file(ent.path())
                            } else if typ.is_symlink() {
                                follow_symlink(ent.path(), include_hidden)
                            } else {
                                0u128
                            }
                        }
                        Err(_) => 0u128,
                    },
                    Err(_) => 0u128,
                })
                .reduce(|a, b| a + b)
            {
                Some(val) => val,
                None => 0u128,
            }
        }
        Err(e) => {
            eprintln!("Error while reading directory {}: {}", path, e);
            0u128
        }
    }
}

fn main() {
    let args = clap::Command::new("line_count")
        .author("Rafael Sundorf <rafael.sundorf@gmail.com>")
        .arg(
            clap::Arg::new("location")
                .short('l')
                .long("location")
                .help("The directory path to loop through or file")
                .default_value("."),
        )
        .arg(
            clap::Arg::new("include-hidden")
                .short('i')
                .long("include-hidden")
                .help("Includes hidden directories")
                .num_args(0..1),
        )
        .get_matches();

    let path: &String = args.get_one("location").unwrap();
    let include = args.get_flag("include-hidden");
    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("Couldn't read metadata of path {}: {}", path, e);
            return;
        }
    };
    let lines = if metadata.is_file() {
        let reader = match std::fs::File::open(path) {
            Ok(file) => std::io::BufReader::new(file),
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        reader.lines().count() as u128
    } else if metadata.is_symlink() {
        follow_symlink(std::path::PathBuf::from(path), include)
    } else {
        read_dir(path, include)
    };
    println!("{}", lines);
}
