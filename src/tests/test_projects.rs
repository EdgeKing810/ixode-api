#[cfg(test)]
use crate::components::{
    io::remove_file,
    project::{fetch_all_projects, save_all_projects, Project},
};

#[test]
fn test_projects() {
    let file_name: &str = "data/projects_test.txt";
    remove_file(file_name.to_string());

    let mut all_projects = fetch_all_projects(file_name.to_string(), &String::new());
    println!("{:#?}", all_projects);

    let test_project = Project::create(
        &mut all_projects,
        "test",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(test_project, Ok(()));

    let test_project2 = Project::create(
        &mut all_projects,
        "test ",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: new_id contains an invalid character"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test *** Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: name contains an invalid character"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects-",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: api_path contains an invalid character"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/Projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: api_path should not contain uppercase alphabetical character(s)"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: id is already in use"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project.",
        "/api/v1/projects",
    );
    assert_eq!(
        test_project2,
        Err(String::from("Error: api_path is already in use"))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a test project;",
        "/api/v1/projects2",
    );
    assert_eq!(
        test_project2,
        Err(String::from(
            "Error: description contains an invalid character"
        ))
    );

    let test_project2 = Project::create(
        &mut all_projects,
        "test2",
        "Test Project",
        "This is a new test project.",
        "/api/v1/projects2",
    );
    assert_eq!(test_project2, Ok(()));

    let test2_id = String::from("test2");

    let test_project3 = Project::update_name(&mut all_projects, &test2_id, "Test Project 3");
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_description(
        &mut all_projects,
        &test2_id,
        "This is a new test project (3).",
    );
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_api_path(&mut all_projects, &test2_id, "/api/v1/projects3");
    assert_eq!(test_project3, Ok(()));

    let test_project3 = Project::update_id(&mut all_projects, &test2_id, "test3");
    assert_eq!(test_project3, Ok(()));

    save_all_projects(&all_projects, String::from(file_name), &String::new());
}
