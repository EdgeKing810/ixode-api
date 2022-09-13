#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::core::core_body_data::BodyData;

fn make_core_one(all_pairs: &mut Vec<crate::components::routing::core::core_body_data::BodyData>) {
    crate::components::routing::core::core_body_data::BodyData::create(all_pairs, "uid", "STRING")
        .unwrap();

    crate::components::routing::core::core_body_data::BodyData::create(
        all_pairs,
        "profileID",
        "STRING",
    )
    .unwrap();

    crate::components::routing::core::core_body_data::BodyData::create(
        all_pairs, "status", "BOOLEAN",
    )
    .unwrap();
}

fn get_core_str_one() -> String {
    "ADD BODY pair [uid,STRING]\nADD BODY pair [profileID,STRING]\nADD BODY pair [status,BOOLEAN]"
        .to_string()
}

#[test]
pub fn run_routing_core_body_data_one() {
    println!("---> Running Routing Core Body Data One");
    // ADD BODY pair [uid,STRING]\nADD BODY pair [profileID,STRING]\nADD BODY pair [status,BOOLEAN]

    let mut all_pairs = Vec::<BodyData>::new();
    make_core_one(&mut all_pairs);

    assert_eq!(get_core_str_one(), BodyData::stringify(&all_pairs, false));
}

#[test]
pub fn run_routing_core_body_data_two() {
    println!("---> Running Routing Core Body Data Two");

    // [
    //     BodyData {
    //         id: "uid",
    //         bdtype: STRING
    //     },
    //     BodyData {
    //         id: "profileID",
    //         bdtype: STRING
    //     },
    //     BodyData {
    //         id: "status",
    //         bdtype: BOOLEAN
    //     }
    // ]

    let mut all_pairs = Vec::<BodyData>::new();
    let mut all_pairs_duplicate = Vec::<BodyData>::new();

    for line in get_core_str_one().split("\n") {
        BodyData::from_string(&mut all_pairs, line, false).unwrap();
    }

    make_core_one(&mut all_pairs_duplicate);

    assert_eq!(all_pairs_duplicate, all_pairs);
}
