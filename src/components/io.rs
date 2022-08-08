use crate::{components::encryption::EncryptionKey, utils::get_root_data_dir};
use std::{
    fs,
    fs::{create_dir, read_dir, remove_dir_all, rename, File},
    io::prelude::*,
    io::BufReader,
};

pub fn fetch_file(path: String, encryption_key: &String) -> String {
    let file = File::open(&path);
    let mut content = String::new();
    let mut final_content = String::new();

    ensure_file_exists(&path);

    match file {
        Ok(f) => {
            let mut buf_reader = BufReader::new(f);
            let read_file = buf_reader.read_to_string(&mut content);
            if let Err(e) = &read_file {
                println!("Error occured while reading file at {}: {}", path, e);
            }

            if let Ok(_) = read_file {
                if encryption_key.len() > 2 {
                    let broken_content = content
                        .split("\n")
                        .filter(|line| line.chars().count() >= 3)
                        .collect::<Vec<&str>>();

                    if broken_content.len() > 0
                        && broken_content[0] == String::from(";|encrypted|;")
                    {
                        for bc in broken_content {
                            if bc == String::from(";|encrypted|;") {
                                continue;
                            }

                            let decrypted_data =
                                EncryptionKey::decrypt(bc.to_string(), encryption_key);

                            if let Ok(d) = &decrypted_data {
                                final_content = format!(
                                    "{}{}{}",
                                    final_content,
                                    if final_content.chars().count() > 1 {
                                        "\n"
                                    } else {
                                        ""
                                    },
                                    d.0
                                );
                            }

                            if let Err(e) = &decrypted_data {
                                println!("Error: Failed decrypting {} ({})", path, e);
                            }
                        }
                    } else {
                        final_content = content.clone();
                    }
                } else {
                    final_content = content.clone();
                }
            }
        }
        _ => {}
    }

    final_content
}

pub fn ensure_file_exists(path: &String) {
    let file = File::open(&path);

    match file {
        Err(_) => {
            let create_file = File::create(&path);
            if let Err(e) = create_file {
                println!("Error occured while creating file at {}: {}", &path, e);
            }
        }
        _ => {}
    }
}

pub fn save_file(path: String, data: String, encryption_key: &String) {
    ensure_file_exists(&path);
    let file = File::create(&path);

    let mut final_data = data.clone();
    if encryption_key.len() > 2 {
        final_data = String::from(";|encrypted|;");

        let broken_data = data.split("\n").filter(|line| line.chars().count() >= 3);

        for bd in broken_data {
            let encrypted_data = EncryptionKey::encrypt(bd.to_string(), encryption_key);
            final_data = format!("{}\n{}", final_data, encrypted_data);
        }
    }

    if let Ok(mut f) = file {
        let write_file = f.write_all(final_data.as_bytes());

        if let Err(e) = write_file {
            println!("Error occured while writing file at {}: {}", &path, e);
        }
    }
}

pub fn remove_file(path: String) {
    ensure_file_exists(&path);
    let remove_file_result = fs::remove_file(&path);
    if let Err(e) = remove_file_result {
        println!("Error while removing file: {} ({})", e, path);
    }
}

pub fn ensure_directory_exists(path: &String) {
    let directory = read_dir(&path);

    match directory {
        Err(_) => {
            println!("path: {}", path);
            let create_directory = create_dir(&path);
            if let Err(e) = create_directory {
                println!("Error occured while creating directory at {}: {}", &path, e);
            }
        }
        _ => {}
    }
}

pub fn remove_directory(path: &String) {
    let root_dir = get_root_data_dir();
    if !path.contains(&root_dir) {
        print!("Unallowed attempt to remove: {}", path);
        return;
    }

    ensure_directory_exists(&path);
    let remove_directory_result = remove_dir_all(&path);
    if let Err(e) = remove_directory_result {
        println!("Error while removing directory: {} ({})", e, path);
    }
}

pub fn rename_directory(old_path: &String, path: &String) {
    let rename_directory_result = rename(&old_path, &path);
    if let Err(e) = rename_directory_result {
        println!(
            "Error while renaming directory: {} (from: {}) (to: {})",
            e, old_path, path
        );
    }
}
