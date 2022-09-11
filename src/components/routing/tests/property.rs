#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::property_block::PropertyBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_property::Property;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

#[test]
pub fn run_routing_property_one() {
    println!("---> Running Routing Property One");
    // PROPERTY (12,4) [currentProfile_blocked] ([ref,OTHER,currentProfile]|apply=blocked)

    let mut all_blocks = Vec::<PropertyBlock>::new();
    let data = RefData::create(true, "OTHER", "currentProfile").unwrap();

    if let Err(e) = PropertyBlock::create(
        &mut all_blocks,
        12,
        4,
        "currentProfile_blocked",
        data,
        "blocked",
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    println!("{}", PropertyBlock::to_string(all_blocks[0].clone()));
}

#[test]
pub fn run_routing_property_two() {
    println!("---> Running Routing Property Two");

    // PropertyBlock {
    //     global_index: 12,
    //     block_index: 4,
    //     local_name: "currentProfile_blocked",
    //     data: RefData {
    //         ref_var: true,
    //         rtype: OTHER,
    //         data: "currentProfile",
    //     },
    //     apply: "blocked",
    // }

    let block_str =
        "PROPERTY (12,4) [currentProfile_blocked] ([ref,OTHER,currentProfile]|apply=blocked)";

    let mut all_blocks = Vec::<PropertyBlock>::new();
    PropertyBlock::from_string(&mut all_blocks, block_str).unwrap();

    println!("{:#?}", all_blocks[0].clone());
}

#[test]
pub fn run_routing_property_three() {
    println!("---> Running Routing Property Three");
    // PROPERTY (41,9) [currentUser] ([ref,OTHER,currentUsers]|apply=GET_FIRST)

    let mut all_blocks = Vec::<PropertyBlock>::new();
    let data = RefData::create(true, "OTHER", "currentUsers").unwrap();

    if let Err(e) = PropertyBlock::create(&mut all_blocks, 41, 9, "currentUser", data, "GET_FIRST")
    {
        println!("Error: {:#?}", e);
        return;
    }

    println!("{}", PropertyBlock::to_string(all_blocks[0].clone()));
}

#[test]
pub fn run_routing_property_four() {
    println!("---> Running Routing Property Four");

    // PropertyBlock {
    //     global_index: 41,
    //     block_index: 9,
    //     local_name: "currentUser",
    //     data: RefData {
    //         ref_var: true,
    //         rtype: OTHER,
    //         data: "currentUsers",
    //     },
    //     apply: GET_FIRST,
    // }

    let block_str = "PROPERTY (41,9) [currentUser] ([ref,OTHER,currentUsers]|apply=GET_FIRST)";

    let mut all_blocks = Vec::<PropertyBlock>::new();
    PropertyBlock::from_string(&mut all_blocks, block_str).unwrap();

    println!("{:#?}", all_blocks[0].clone());
}
