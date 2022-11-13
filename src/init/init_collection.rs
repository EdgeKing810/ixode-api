use crate::{
    components::{
        collection::{fetch_all_collections, save_all_collections, Collection},
        custom_structure::CustomStructure,
        mapping::{get_file_name, Mapping},
        structure::Structure,
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_collections(mappings: &Vec<Mapping>) -> Vec<Collection> {
    let all_collections_path = get_file_name("collections", mappings);
    let mut all_collections = Vec::<Collection>::new();

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    if let Err(e) = all_collections_path {
        println!("{}", e);
        return all_collections;
    }

    all_collections = fetch_all_collections(
        all_collections_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e.1);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "Title of a post",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
            true,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "cover_image",
            "Cover Image",
            "Cover Image picture",
            "media",
            "https://api.kinesis.world/public/banner_purple.png",
            0,
            200,
            false,
            false,
            "",
            false,
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "content",
            "Content",
            "Actual content of the post",
            "markdown",
            "[ Content goes here ]",
            10,
            2000,
            false,
            false,
            "",
            false,
            true,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "views",
            "Views",
            "Number of people that have viewed a post",
            "number",
            "0",
            0,
            9999,
            false,
            false,
            "",
            false,
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "published",
            "Published",
            "Whether the blog post is published or not",
            "boolean",
            "false",
            0,
            5,
            false,
            false,
            "",
            true,
            true,
        )
        .unwrap();
        Collection::set_structures(&mut all_collections, &"posts".to_string(), all_structures)
            .unwrap();

        let mut all_custom_structures = Vec::<CustomStructure>::new();
        let mut tmp_structures = Vec::<Structure>::new();

        Structure::create(
            &mut tmp_structures,
            "uid",
            "UID",
            "ID of the User posting a comment",
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
            true,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "Actual content of the comment",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
            true,
        )
        .unwrap();

        CustomStructure::create(
            &mut all_custom_structures,
            "comment",
            "comment",
            "To store comments",
        )
        .unwrap();
        CustomStructure::set_structures(
            &mut all_custom_structures,
            &"comment".to_string(),
            tmp_structures,
        )
        .unwrap();
        Collection::set_custom_structures(
            &mut all_collections,
            &"posts".to_string(),
            all_custom_structures,
        )
        .unwrap();

        save_all_collections(
            &all_collections,
            all_collections_path.unwrap(),
            &get_encryption_key(&mappings, &pass),
        );
    }

    all_collections
}
