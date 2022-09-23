#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::condition_block::ConditionBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_condition::Condition;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_fail_obj::FailObj;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::condition_block::ConditionBlock>,
) {
    let fail_obj = match FailObj::create(403, "Error: You cannot follow yourself") {
        Ok(fail_obj) => fail_obj,
        Err(e) => {
            println!("{}", e.1);
            FailObj::default()
        }
    };

    if let Err(e) = crate::components::routing::blocks::condition_block::ConditionBlock::create(
        all_blocks,
        0,
        0,
        "FAIL",
        Some(fail_obj),
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_conditions = Vec::<Condition>::new();

    let left = RefData::create(true, "STRING", "uid").unwrap();
    let right = RefData::create(true, "STRING", "profileID").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "AND");

    let left = RefData::create(true, "BOOLEAN", "status").unwrap();
    let right = RefData::create(false, "BOOLEAN", "true").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "NONE");

    crate::components::routing::blocks::condition_block::ConditionBlock::set_conditions(
        all_blocks,
        0,
        all_conditions,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "CONDITION (0,0) [FAIL] [403,Error: You cannot follow yourself] ([ref,STRING,uid]|EQUAL_TO|[ref,STRING,profileID]|not=false|next=AND)>([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)".to_string()
}

#[test]
pub fn run_routing_condition_one() {
    println!("---> Running Routing Condition One");
    // CONDITION (0,0) [FAIL] [403,Error: You cannot follow yourself] ([ref,STRING,uid]|EQUAL_TO|[ref,STRING,profileID]|not=false|next=AND)>([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)

    let mut all_blocks = Vec::<ConditionBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        ConditionBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_condition_two() {
    println!("---> Running Routing Condition Two");

    // ConditionBlock {
    //     global_index: 0,
    //     block_index: 0,
    //     conditions: [
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "uid",
    //             },
    //             right: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "profileID",
    //             },
    //             condition_type: EQUAL_TO,
    //             not: false,
    //             next: AND,
    //         },
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: BOOLEAN,
    //                 data: "status",
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: BOOLEAN,
    //                 data: "true",
    //             },
    //             condition_type: EQUAL_TO,
    //             not: false,
    //             next: NONE,
    //         },
    //     ],
    //     action: FAIL,
    //     fail: FailObj {
    //         status: 403,
    //         message: "Error: You cannot follow yourself",
    //     },
    // }

    let mut all_blocks = Vec::<ConditionBlock>::new();
    ConditionBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<ConditionBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
