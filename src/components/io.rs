use crate::{
    components::encryption::EncryptionKey,
    utils::io::{
        auto_check_lock, auto_create_lock, auto_release_lock, get_root_data_dir, obtain_lock_name,
    },
};
use std::{
    fs,
    fs::{copy, create_dir, read_dir, remove_dir_all, File},
    io::prelude::*,
    io::BufReader,
};

pub fn fetch_file(path: String, encryption_key: &String) -> String {
    // let lock_name = obtain_lock_name(&path);
    // while auto_check_lock(&lock_name) {
    //     std::thread::sleep(std::time::Duration::from_millis(200));
    // }
    // auto_create_lock(&lock_name);

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

    // auto_release_lock(&lock_name);
    final_content
}

pub fn ensure_file_exists(path: &String) {
    // println!("running ensure_file_exists: {}", path);
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
    println!("running save_file: {}", path);

    let mut lock_name = String::new();
    if !path.contains("lock") {
        lock_name = obtain_lock_name(&path);
        while auto_check_lock(&lock_name) {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        auto_create_lock(&lock_name);
    }

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

    auto_release_lock(&lock_name);
}

pub fn remove_file(path: String) {
    println!("running remove_file: {}", path);

    // let lock_name = obtain_lock_name(&path);
    // while auto_check_lock(&lock_name) {
    //     std::thread::sleep(std::time::Duration::from_millis(200));
    // }
    // auto_create_lock(&lock_name);

    ensure_file_exists(&path);
    let remove_file_result = fs::remove_file(&path);
    if let Err(e) = remove_file_result {
        println!("Error while removing file: {} ({})", e, path);
    }

    // auto_release_lock(&lock_name);
}

pub fn ensure_directory_exists(path: &String) {
    println!("running ensure_directory_exists: {}", path);
    let directory = read_dir(&path);

    match directory {
        Err(_) => {
            let create_directory = create_dir(&path);
            if let Err(e) = create_directory {
                println!("Error occured while creating directory at {}: {}", &path, e);
            }
        }
        _ => {}
    }
}

pub fn remove_directory(path: &String) {
    println!("running remove_directory: {}", path);
    let root_dir = get_root_data_dir();
    if !path.contains(&root_dir) {
        print!("Unallowed attempt to remove: {}", path);
        return;
    }

    ensure_directory_exists(&path);

    for entry in read_dir(&path).unwrap() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();

            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    for int_entry in read_dir(&entry_path.to_str().unwrap().to_string()).unwrap() {
                        if let Ok(int_entry) = int_entry {
                            let int_entry_path = int_entry.path();

                            if let Ok(ift) = int_entry.file_type() {
                                if ift.is_dir() {
                                    remove_directory(&int_entry_path.to_str().unwrap().to_string());
                                } else {
                                    let lock_name = obtain_lock_name(
                                        &int_entry_path.clone().to_str().unwrap().to_string(),
                                    );
                                    while auto_check_lock(&lock_name) {
                                        std::thread::sleep(std::time::Duration::from_millis(200));
                                    }
                                    auto_create_lock(&lock_name);

                                    remove_file(int_entry_path.to_str().unwrap().to_string());

                                    auto_release_lock(&lock_name);
                                }
                            }
                        }
                    }

                    if let Err(e) = remove_dir_all(entry_path.clone()) {
                        println!("Error while removing directory: {} ({:?})", e, entry_path);
                    }
                } else {
                    let lock_name =
                        obtain_lock_name(&entry_path.clone().to_str().unwrap().to_string());
                    while auto_check_lock(&lock_name) {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                    auto_create_lock(&lock_name);

                    remove_file(entry_path.to_str().unwrap().to_string());

                    auto_release_lock(&lock_name);
                }
            }
        }
    }

    if let Err(e) = remove_dir_all(path) {
        println!("Error while removing directory: {} ({})", e, path);
    }
}

pub fn rename_directory(old_path: &String, path: &String) {
    println!("running rename_directory: {} -> {}", old_path, path);
    if old_path == path {
        return;
    }

    ensure_directory_exists(&path);
    ensure_directory_exists(&old_path);

    for entry in read_dir(&old_path).unwrap() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dest = format!("{}/{}", path, file_name.to_str().unwrap());

            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    ensure_directory_exists(&dest);
                    for int_entry in read_dir(&entry_path.to_str().unwrap().to_string()).unwrap() {
                        if let Ok(int_entry) = int_entry {
                            let int_entry_path = int_entry.path();
                            let int_file_name = int_entry.file_name();
                            let int_file_dest =
                                format!("{}/{}", dest, int_file_name.to_str().unwrap());

                            if let Ok(ift) = int_entry.file_type() {
                                if !ift.is_dir() {
                                    let lock_name = obtain_lock_name(
                                        &int_entry_path.clone().to_str().unwrap().to_string(),
                                    );
                                    while auto_check_lock(&lock_name) {
                                        std::thread::sleep(std::time::Duration::from_millis(200));
                                    }
                                    auto_create_lock(&lock_name);

                                    if let Err(e) =
                                        copy(int_entry_path.clone(), int_file_dest.clone())
                                    {
                                        println!(
                                            "Error while copying file: {} ({:?} => {})",
                                            e, int_entry_path, dest
                                        );
                                    } else {
                                        remove_file(int_entry_path.to_str().unwrap().to_string());
                                    }
                                    auto_release_lock(&lock_name);
                                }
                            }
                        }
                    }
                    remove_directory(&entry_path.to_str().unwrap().to_string())
                } else {
                    let lock_name =
                        obtain_lock_name(&entry_path.clone().to_str().unwrap().to_string());
                    while auto_check_lock(&lock_name) {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                    auto_create_lock(&lock_name);

                    if let Err(e) = copy(entry_path.clone(), dest.clone()) {
                        println!(
                            "Error while copying file: {} ({:?} => {})",
                            e, entry_path, dest
                        );
                    } else {
                        remove_file(entry_path.to_str().unwrap().to_string());
                    }
                    auto_release_lock(&lock_name);
                }
            }
        }
    }

    remove_directory(old_path);
}
