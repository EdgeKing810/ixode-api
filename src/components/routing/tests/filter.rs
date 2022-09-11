#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::filter_block::FilterBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_filter::Filter;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

#[test]
pub fn run_routing_filter_one() {
    println!("---> Running Routing Filter One");
    // FILTER (4,2) [currentProfiles,profiles,uid] ([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)

    let mut all_blocks = Vec::<FilterBlock>::new();
    if let Err(e) = FilterBlock::create(&mut all_blocks, 4, 2, "currentProfiles", "profiles", "uid")
    {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_filters = Vec::<Filter>::new();

    let right = RefData::create(true, "STRING", "uid").unwrap();
    Filter::create(&mut all_filters, right, "EQUAL_TO", false, "NONE");

    FilterBlock::set_filters(&mut all_blocks, 4, all_filters).unwrap();

    println!("{}", FilterBlock::to_string(all_blocks[0].clone()));
}

#[test]
pub fn run_routing_filter_two() {
    println!("---> Running Routing Filter Two");

    // FilterBlock {
    //     global_index: 4,
    //     block_index: 2,
    //     local_name: "currentProfiles",
    //     ref_var: "profiles",
    //     ref_property: "uid",
    //     filters: [
    //         Filter {
    //             right: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "uid",
    //             },
    //             operation_type: EQUAL_TO,
    //             not: false,
    //             next: NONE,
    //         },
    //     ],
    // }

    let block_str = "FILTER (4,2) [currentProfiles,profiles,uid] ([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)";

    let mut all_blocks = Vec::<FilterBlock>::new();
    FilterBlock::from_string(&mut all_blocks, block_str).unwrap();

    println!("{:#?}", all_blocks[0].clone());
}
