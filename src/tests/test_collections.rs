#[cfg(test)]
use crate::{
    collection::{fetch_all_collections, save_all_collections, Collection},
    custom_structures::CustomStructure,
    io::remove_file,
    structures::Structure,
};

#[test]
fn test_correct_collection() {
    let file_name: &str = "data/collection_ok_test.txt";
    remove_file(file_name.to_string());

    let mut all_collections = Vec::<Collection>::new();
    all_collections = fetch_all_collections(file_name.to_string(), &String::new());

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "cover_image",
            "Cover Image",
            "media",
            "https://test.image.com",
            0,
            200,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "content",
            "Content",
            "richtext",
            "[ Content goes here ]",
            30,
            2000,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "views",
            "Views",
            "number",
            "0",
            0,
            9999,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "comment",
            "Comments",
            "comment",
            "0",
            0,
            9999,
            false,
            false,
            "",
            true,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "published",
            "Published",
            "boolean",
            "false",
            0,
            5,
            false,
            false,
            "",
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
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        CustomStructure::create(&mut all_custom_structures, "comment", "comment").unwrap();
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
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}

#[test]
fn test_incorrect_collection() {
    let file_name: &str = "data/collection_err_test.txt";
    remove_file(file_name.to_string());

    let mut all_collections = Vec::<Collection>::new();
    all_collections = fetch_all_collections(file_name.to_string(), &String::new());

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        let test_structure = Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        );
        assert_eq!(
            test_structure,
            Err(String::from("Error: id is already in use"))
        );

        let test_structure = Structure::create(
            &mut all_structures,
            "title2=",
            "Title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        );
        assert_eq!(
            test_structure,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_structure =
            Structure::update_id(&mut all_structures, &"title2".to_string(), "title3");
        assert_eq!(
            test_structure,
            Err(String::from("Error: Structure not found"))
        );

        let test_structure =
            Structure::update_name(&mut all_structures, &"title".to_string(), "Title-");
        assert_eq!(
            test_structure,
            Err(String::from("Error: name contains an invalid character"))
        );

        let test_structure =
            Structure::update_type(&mut all_structures, &"title".to_string(), "test;");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: stype_txt contains an invalid character"
            ))
        );

        let test_structure =
            Structure::update_default(&mut all_structures, &"title".to_string(), "test@");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: default_val contains an invalid character"
            ))
        );

        let test_structure =
            Structure::update_regex(&mut all_structures, &"title".to_string(), "^;$");
        assert_eq!(
            test_structure,
            Err(String::from(
                "Error: regex_pattern contains an invalid character"
            ))
        );

        Collection::set_structures(&mut all_collections, &"posts".to_string(), all_structures)
            .unwrap();

        let mut all_custom_structures = Vec::<CustomStructure>::new();
        let mut tmp_structures = Vec::<Structure>::new();

        Structure::create(
            &mut tmp_structures,
            "uid",
            "UID",
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        CustomStructure::create(&mut all_custom_structures, "comment", "comment").unwrap();
        CustomStructure::set_structures(
            &mut all_custom_structures,
            &"comment".to_string(),
            tmp_structures,
        )
        .unwrap();

        let test_custom_structure =
            CustomStructure::create(&mut all_custom_structures, "comment", "comment");
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: id is already in use"))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment2".to_string(),
            "comment3",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: Custom Structure not found"))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment*",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_custom_structure = CustomStructure::update_name(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment^^^",
        );
        assert_eq!(
            test_custom_structure,
            Err(String::from("Error: name contains an invalid character"))
        );

        Collection::set_custom_structures(
            &mut all_collections,
            &"posts".to_string(),
            all_custom_structures,
        )
        .unwrap();

        let test_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        assert_eq!(
            test_collection,
            Err(String::from("Error: id is already in use"))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts2".to_string(), "posts3");
        assert_eq!(
            test_collection,
            Err(String::from("Error: Collection not found"))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts".to_string(), "posts;");
        assert_eq!(
            test_collection,
            Err(String::from("Error: new_id contains an invalid character"))
        );

        let test_collection =
            Collection::update_project_id(&mut all_collections, &"posts".to_string(), "konnect;");
        assert_eq!(
            test_collection,
            Err(String::from(
                "Error: project_id contains an invalid character"
            ))
        );

        let test_collection =
            Collection::update_name(&mut all_collections, &"posts".to_string(), "Pos>ts");
        assert_eq!(
            test_collection,
            Err(String::from("Error: name contains an invalid character"))
        );

        let test_collection = Collection::update_description(
            &mut all_collections,
            &"posts".to_string(),
            "To store blog posts@.",
        );
        assert_eq!(
            test_collection,
            Err(String::from(
                "Error: description contains an invalid character"
            ))
        );
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}
