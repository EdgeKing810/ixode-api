#[cfg(test)]
use crate::components::{
    collection::{fetch_all_collections, save_all_collections, Collection},
    custom_structures::CustomStructure,
    io::{ensure_directory_exists, remove_file},
    structures::Structure,
};

#[test]
fn test_correct_collection() {
    ensure_directory_exists(&String::from("/tmp/data"));
    ensure_directory_exists(&String::from("/tmp/data/projects"));
    ensure_directory_exists(&String::from("/tmp/data/projects/konnect"));

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
            100,
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
            15,
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
            1,
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
            0,
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
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}

#[test]
fn test_incorrect_collection() {
    ensure_directory_exists(&String::from("/tmp/data"));
    ensure_directory_exists(&String::from("/tmp/data/projects"));
    ensure_directory_exists(&String::from("/tmp/data/projects/konnect"));

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
            println!("{}", e.1);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "a title",
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

        let test_structure = Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "a title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
            true,
        );
        assert_eq!(
            test_structure,
            Err((403, String::from("Error: id is already in use")))
        );

        let test_structure = Structure::create(
            &mut all_structures,
            "title2=",
            "Title",
            "a title",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
            true,
        );
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: new_id contains an invalid character")
            ))
        );

        let test_structure = Structure::create(
            &mut all_structures,
            "title2",
            "Title",
            "a title;",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
            true,
        );
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: description contains an invalid character")
            ))
        );

        let test_structure =
            Structure::update_id(&mut all_structures, &"title2".to_string(), "title3");
        assert_eq!(
            test_structure,
            Err((404, String::from("Error: Structure not found")))
        );

        let test_structure =
            Structure::update_name(&mut all_structures, &"title".to_string(), "Title-");
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: name contains an invalid character")
            ))
        );

        let test_structure =
            Structure::update_description(&mut all_structures, &"title".to_string(), "a title;");
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: description contains an invalid character")
            ))
        );

        let test_structure =
            Structure::update_type(&mut all_structures, &"title".to_string(), "test;");
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: stype_txt contains an invalid character")
            ))
        );

        let test_structure =
            Structure::update_default(&mut all_structures, &"title".to_string(), "test>");
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: default_val contains an invalid character")
            ))
        );

        let test_structure =
            Structure::update_regex(&mut all_structures, &"title".to_string(), "^;$");
        assert_eq!(
            test_structure,
            Err((
                400,
                String::from("Error: regex_pattern contains an invalid character")
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
            "ID of the User posting a comment",
            "uid",
            "",
            0,
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
            0,
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

        let test_custom_structure = CustomStructure::create(
            &mut all_custom_structures,
            "comment",
            "comment",
            "To store comments",
        );
        assert_eq!(
            test_custom_structure,
            Err((403, String::from("Error: id is already in use")))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment2".to_string(),
            "comment3",
        );
        assert_eq!(
            test_custom_structure,
            Err((404, String::from("Error: Custom Structure not found")))
        );

        let test_custom_structure = CustomStructure::update_id(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment*",
        );
        assert_eq!(
            test_custom_structure,
            Err((
                400,
                String::from("Error: new_id contains an invalid character")
            ))
        );

        let test_custom_structure = CustomStructure::update_name(
            &mut all_custom_structures,
            &"comment".to_string(),
            "comment^^^",
        );
        assert_eq!(
            test_custom_structure,
            Err((
                400,
                String::from("Error: name contains an invalid character")
            ))
        );

        let test_custom_structure = CustomStructure::update_description(
            &mut all_custom_structures,
            &"comment".to_string(),
            "To store comments;",
        );
        assert_eq!(
            test_custom_structure,
            Err((
                400,
                String::from("Error: description contains an invalid character")
            ))
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
            Err((403, String::from("Error: id is already in use")))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts2".to_string(), "posts3");
        assert_eq!(
            test_collection,
            Err((404, String::from("Error: Collection not found")))
        );

        let test_collection =
            Collection::update_id(&mut all_collections, &"posts".to_string(), "posts;");
        assert_eq!(
            test_collection,
            Err((
                400,
                String::from("Error: new_id contains an invalid character")
            ))
        );

        let test_collection =
            Collection::update_project_id(&mut all_collections, &"posts".to_string(), "konnect;");
        assert_eq!(
            test_collection,
            Err((
                400,
                String::from("Error: project_id contains an invalid character")
            ))
        );

        let test_collection =
            Collection::update_name(&mut all_collections, &"posts".to_string(), "Pos>ts");
        assert_eq!(
            test_collection,
            Err((
                400,
                String::from("Error: name contains an invalid character")
            ))
        );

        let test_collection = Collection::update_description(
            &mut all_collections,
            &"posts".to_string(),
            "To store blog posts;.",
        );
        assert_eq!(
            test_collection,
            Err((
                400,
                String::from("Error: description contains an invalid character")
            ))
        );
    }
    save_all_collections(&all_collections, file_name.to_string(), &String::new());
}
