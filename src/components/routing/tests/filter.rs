#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::filter_block::FilterBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_filter::Filter;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::filter_block::FilterBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::filter_block::FilterBlock::create(
        all_blocks,
        4,
        2,
        "currentProfiles",
        "profiles",
        "uid",
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_filters = Vec::<Filter>::new();

    let right = RefData::create(true, "STRING", "uid").unwrap();
    Filter::create(&mut all_filters, right, "EQUAL_TO", false, "NONE");

    crate::components::routing::blocks::filter_block::FilterBlock::set_filters(
        all_blocks,
        4,
        all_filters,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "FILTER (4,2) [currentProfiles,profiles,uid] ([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)"
        .to_string()
}

#[test]
pub fn run_routing_filter_one() {
    println!("---> Running Routing Filter One");
    // FILTER (4,2) [currentProfiles,profiles,uid] ([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)

    let mut all_blocks = Vec::<FilterBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        FilterBlock::to_string(all_blocks[0].clone())
    );
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

    let mut all_blocks = Vec::<FilterBlock>::new();
    FilterBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<FilterBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
