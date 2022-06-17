use std::io::BufRead;

fn read_dir(path: &str, include_hidden: bool) -> u128 {
    match std::fs::read_dir(path) {
        Ok(res) => {
            match res.map(|entry| {
                match entry {
                    Ok(ent) => {
                        match ent.file_type() {
                            Ok(typ) => {
                                if typ.is_dir() {
                                    if (ent.file_name().to_str().unwrap().starts_with(".") && include_hidden) || !ent.file_name().to_str().unwrap().starts_with(".")  {
                                        read_dir(ent.path().to_str().unwrap(), include_hidden)
                                    } else {
                                        0 as u128
                                    }
                                } else if typ.is_file() {
                                    match std::fs::File::open(ent.path().to_str().unwrap()) {
                                        Ok(file) => {
                                            let reader = std::io::BufReader::new(file);
                                            let mut lines = 0 as u128;
                                            for line in reader.lines() {
                                                match line {
                                                    Ok(_) => lines += 1 as u128,
                                                    Err(_) => {
                                                        return 0 as u128;
                                                    }
                                                }
                                            }
                                            lines
                                        },
                                        Err(e) => {
                                            eprintln!("{}", e);
                                            0 as u128
                                        }
                                    }
                                } else {
                                    0 as u128
                                }
                            },
                            Err(_) => 0 as u128
                        }
                    },
                    Err(_) => {
                        0 as u128
                    }
                }
            }).reduce(|a, b| {
                a + b
            }) {
                Some(val) => val,
                None => 0 as u128
            }
        },
        Err(e) => {
            eprintln!("Error while reading directory {}: {}", path, e);
            0 as u128
        }
    }
}

fn main() {
    let args = clap::App::new("line_count")
        .author("Rafael Sundorf <rafael.sundorf@gmail.com>")
        .arg(clap::Arg::new("location").short('l').long("location").help("The directory path to loop through or file").default_value(".").forbid_empty_values(true))
        .arg(clap::Arg::new("include-hidden").short('i').long("include-hidden").help("Includes hidden directories").takes_value(false))
        .get_matches();

    let path = args.value_of("location").unwrap();
    let include = args.is_present("include-hidden");
    let folder = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let metadata = match folder.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("Couldn't read metadata of path {}: {}", path, e);
            return;
        }
    };
    let lines = if metadata.is_file() {
        let reader = std::io::BufReader::new(folder);
        reader.lines().count() as u128
    } else {
        read_dir(path, include)
    };
    println!("{}", lines);
}
