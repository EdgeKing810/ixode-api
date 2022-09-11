#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::assignment_block::AssignmentBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_condition::Condition;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_operation::Operation;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::assignment_block::AssignmentBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::assignment_block::AssignmentBlock::create(
        all_blocks,
        39,
        9,
        "notificationContent",
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_conditions = Vec::<Condition>::new();
    let mut all_operations = Vec::<Operation>::new();

    let left = RefData::create(true, "STRING", "targetProfile_account_type").unwrap();
    let right = RefData::create(false, "STRING", "public").unwrap();
    Condition::create(
        &mut all_conditions,
        left,
        right,
        "NOT_EQUAL_TO",
        false,
        "NONE",
    );

    let left = RefData::create(true, "STRING", "notificationContent").unwrap();
    let right = RefData::create(false, "STRING", " has requested to follow you").unwrap();
    Operation::create(&mut all_operations, left, right, "ADDITION", false, "NONE");

    crate::components::routing::blocks::assignment_block::AssignmentBlock::set_conditions(
        all_blocks,
        39,
        all_conditions,
    )
    .unwrap();
    crate::components::routing::blocks::assignment_block::AssignmentBlock::set_operations(
        all_blocks,
        39,
        all_operations,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "ASSIGN (39,9) [notificationContent] {([ref,STRING,notificationContent]|ADDITION|[,STRING, has requested to follow you]|not=false|next=NONE)} ([ref,STRING,targetProfile_account_type]|NOT_EQUAL_TO|[,STRING,public]|not=false|next=NONE)".to_string()
}

#[test]
pub fn run_routing_assignment_one() {
    println!("---> Running Routing Assignment One");
    // ASSIGN (39,9) [notificationContent] {([ref,STRING,notificationContent]|ADDITION|[,STRING, has requested to follow you]|not=false|next=NONE)} ([ref,STRING,targetProfile_account_type]|NOT_EQUAL_TO|[,STRING,public]|not=fasle|next=NONE)

    let mut all_blocks = Vec::<AssignmentBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        AssignmentBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_assignment_two() {
    println!("---> Running Routing Assignment Two");

    // AssignmentBlock {
    //     global_index: 39,
    //     block_index: 9,
    //     local_name: "notificationContent",
    //     conditions: [
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "targetProfile_account_type",
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: STRING,
    //                 data: "public",
    //             },
    //             condition_type: NOT_EQUAL_TO,
    //             not: false,
    //             next: NONE,
    //         },
    //     ],
    //     operations: [
    //         Operation {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "notificationContent",
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: STRING,
    //                 data: " has requested to follow you",
    //             },
    //             operation_type: ADDITION,
    //             not: false,
    //             next: NONE,
    //         },
    //     ],
    // }

    let mut all_blocks = Vec::<AssignmentBlock>::new();
    AssignmentBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<AssignmentBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
