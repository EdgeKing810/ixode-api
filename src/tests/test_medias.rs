#[cfg(test)]
use crate::components::{
    io::remove_file,
    media::{fetch_all_medias, save_all_medias, Media},
};

#[test]
fn test_medias() {
    let file_name: &str = "data/medias_test.txt";
    remove_file(file_name.to_string());

    let mut all_medias = fetch_all_medias(file_name.to_string(), &String::new());
    println!("{:#?}", all_medias);

    let test_media = Media::create(&mut all_medias, "test", "media.png");
    assert_eq!(test_media, Ok(()));

    let test_media2 = Media::create(&mut all_medias, "test ", "media.png");
    assert_eq!(
        test_media2,
        Err((
            400,
            String::from("Error: new_id contains an invalid character")
        ))
    );

    let test_media2 = Media::create(&mut all_medias, "test2", "media^png");
    assert_eq!(
        test_media2,
        Err((
            400,
            String::from("Error: name contains an invalid character")
        ))
    );

    let test_media2 = Media::create(&mut all_medias, "test", "media.png");
    assert_eq!(
        test_media2,
        Err((403, String::from("Error: id is already in use")))
    );

    let test_media2 = Media::create(&mut all_medias, "test2", "media2.png");
    assert_eq!(test_media2, Ok(()));

    let test2_id = String::from("test2");

    let test_media3 = Media::update_name(&mut all_medias, &test2_id, "media3.png");
    assert_eq!(test_media3, Ok(()));

    let test_media3 = Media::update_id(&mut all_medias, &test2_id, "test3");
    assert_eq!(test_media3, Ok(()));

    save_all_medias(&all_medias, String::from(file_name), &String::new());
}
