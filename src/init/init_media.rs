use crate::{
    components::{
        mappings::{get_file_name, Mapping},
        media::{fetch_all_medias, save_all_medias, Media},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_medias(mappings: &Vec<Mapping>) -> Vec<Media> {
    let all_medias_path = get_file_name("medias", mappings);
    let mut all_medias = Vec::<Media>::new();

    if let Err(e) = all_medias_path {
        println!("{}", e);
        return all_medias;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_medias = fetch_all_medias(
        all_medias_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !Media::exist(&all_medias, "logo") {
        let create_logo = Media::create(&mut all_medias, "logo", "logo.png");
        if let Err(e) = create_logo {
            println!("{}", e.1);
        }
    }

    save_all_medias(
        &all_medias,
        all_medias_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_medias
}
