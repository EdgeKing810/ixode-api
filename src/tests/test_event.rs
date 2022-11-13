#[cfg(test)]
use crate::components::{
    event::{fetch_all_events, save_all_events, Event},
    io::remove_file,
};

#[test]
fn main() {
    let file_name: &str = "data/events_test.txt";
    remove_file(file_name.to_string());

    let mut all_events = fetch_all_events(file_name.to_string(), &String::new());
    println!("{:#?}", all_events);

    let test_event = Event::create(&mut all_events, "test", "test event", "/test");
    assert_eq!(test_event, Ok(()));

    let test_event2 = Event::create(&mut all_events, "test2;", "test event2", "/test2");
    assert_eq!(
        test_event2,
        Err((
            400,
            String::from("Error: event_type contains an invalid character")
        ))
    );

    let test_event2 = Event::create(&mut all_events, "test2", "test event2;", "/test2");
    assert_eq!(
        test_event2,
        Err((
            400,
            String::from("Error: description contains an invalid character")
        ))
    );

    let test_event2 = Event::create(&mut all_events, "test2", "test event2", "/test2;");
    assert_eq!(
        test_event2,
        Err((
            400,
            String::from("Error: redirect contains an invalid character")
        ))
    );

    let test_event2 = Event::create(&mut all_events, "test2", "test event2", "/test2");
    assert_eq!(test_event2, Ok(()));

    let test2_id = all_events[1].clone().id;

    let test_event3 = Event::update_event_type(&mut all_events, &test2_id, "test3");
    assert_eq!(test_event3, Ok(()));

    let test_event3 = Event::update_description(&mut all_events, &test2_id, "test event3");
    assert_eq!(test_event3, Ok(()));

    let test_event3 = Event::update_redirect(&mut all_events, &test2_id, "/test3");
    assert_eq!(test_event3, Ok(()));

    save_all_events(&all_events, String::from(file_name), &String::new());
}
