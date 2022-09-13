#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::create_block::CreateBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_condition::Condition;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::create_block::CreateBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::create_block::CreateBlock::create(
        all_blocks,
        47,
        9,
        "notifications",
        "newNotification",
        true,
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_conditions = Vec::<Condition>::new();

    let left = RefData::create(true, "BOOLEAN", "status").unwrap();
    let right = RefData::create(false, "BOOLEAN", "true").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "NONE");

    crate::components::routing::blocks::create_block::CreateBlock::set_conditions(
        all_blocks,
        47,
        all_conditions,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "CREATE (47,9) [notifications,newNotification,true] ([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)".to_string()
}

#[test]
pub fn run_routing_create_one() {
    println!("---> Running Routing Create One");
    // CREATE (47,9) [notifications,newNotification,true] ([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)

    let mut all_blocks = Vec::<CreateBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        CreateBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_create_two() {
    println!("---> Running Routing Create Two");

    // CreateBlock {
    //     global_index: 47,
    //     block_index: 9,
    //     ref_col: "notifications",
    //     ref_object: "newNotification",
    //     save: true, conditions: [
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: BOOLEAN,
    //                 data: "status"
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: BOOLEAN,
    //                 data: "true"
    //             },
    //             condition_type: EQUAL_TO,
    //             not: false,
    //             next: NONE
    //         }
    //     ]
    // }

    let mut all_blocks = Vec::<CreateBlock>::new();
    CreateBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<CreateBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
