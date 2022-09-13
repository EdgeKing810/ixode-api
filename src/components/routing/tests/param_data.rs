#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::core::core_body_data::BodyData;
#[allow(unused_imports)]
use crate::components::routing::core::core_param_data::ParamData;

fn make_core_one() -> crate::components::routing::core::core_param_data::ParamData {
    let mut all_pairs = Vec::<crate::components::routing::core::core_body_data::BodyData>::new();
    let mut new_param_data =
        crate::components::routing::core::core_param_data::ParamData::create("&").unwrap();

    crate::components::routing::core::core_body_data::BodyData::create(
        &mut all_pairs,
        "offset",
        "INTEGER",
    )
    .unwrap();

    crate::components::routing::core::core_body_data::BodyData::create(
        &mut all_pairs,
        "limit",
        "INTEGER",
    )
    .unwrap();

    crate::components::routing::core::core_param_data::ParamData::set_body_data_pairs(
        &mut new_param_data,
        all_pairs,
    );

    new_param_data
}

fn get_core_str_one() -> String {
    "DEFINE PARAMS delimiter &\nADD PARAMS pair [offset,INTEGER]\nADD PARAMS pair [limit,INTEGER]"
        .to_string()
}

#[test]
pub fn run_routing_core_param_data_one() {
    println!("---> Running Routing Core Param Data One");
    // DEFINE PARAMS delimiter &\nADD PARAMS pair [offset,INTEGER]\nADD PARAMS pair [limit,INTGER]

    let param_data = make_core_one();

    assert_eq!(get_core_str_one(), ParamData::to_string(param_data));
}

#[test]
pub fn run_routing_core_param_data_two() {
    println!("---> Running Routing Core Param Data Two");

    // ParamData {
    //     delimiter: "&",
    //     pairs: [
    //         BodyData {
    //             id: "offset",
    //             bdtype: INTEGER
    //         },
    //         BodyData {
    //             id: "limit",
    //             bdtype: OTHER
    //         }
    //     ]
    // }

    let param_data = ParamData::from_string(&get_core_str_one()).unwrap();
    let param_data_duplicate = make_core_one();

    assert_eq!(param_data_duplicate, param_data);
}
