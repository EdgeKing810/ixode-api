#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::template_block::TemplateBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_condition::Condition;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::template_block::TemplateBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::template_block::TemplateBlock::create(
        all_blocks,
        60,
        12,
        "message",
        "This is a test message that has been created by {} on {}",
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_data = Vec::<RefData>::new();

    all_data.push(RefData::create(true, "STRING", "uid").unwrap());
    all_data.push(RefData::create(true, "STRING", "timestamp").unwrap());

    crate::components::routing::blocks::template_block::TemplateBlock::set_data(
        all_blocks, 60, all_data,
    )
    .unwrap();

    let mut all_conditions = Vec::<Condition>::new();

    let left = RefData::create(true, "BOOLEAN", "success").unwrap();
    let right = RefData::create(false, "BOOLEAN", "true").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "NONE");

    crate::components::routing::blocks::template_block::TemplateBlock::set_conditions(
        all_blocks,
        60,
        all_conditions,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "TEMPLATE (60,12) [message] (data=[ref,STRING,uid]>[ref,STRING,timestamp]) {conditions=([ref,BOOLEAN,success]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)} template=This is a test message that has been created by {} on {}".to_string()
}

#[test]
pub fn run_routing_template_one() {
    println!("---> Running Routing Template One");
    // TEMPLATE (60,12) [message] (data=[ref,STRING,uid]>[ref,STRING,timestamp]) {conditions=([ref,BOOLEAN,success]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=NONE)} template=This is a test message that has been created by {} on {}

    let mut all_blocks = Vec::<TemplateBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        TemplateBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_template_two() {
    println!("---> Running Routing Template Two");

    // TemplateBlock {
    //     global_index: 60,
    //     block_index: 12,
    //     local_name: "message",
    //     template: "This is a test message that has been created by {} on {}",
    //     data: [
    //         RefData {
    //             ref_var: true,
    //             rtype: STRING,
    //             data: "uid"
    //         },
    //         RefData {
    //             ref_var: true,
    //             rtype: STRING,
    //             data: "timestamp"
    //         }
    //     ],
    //     conditions: [
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: BOOLEAN,
    //                 data: "success"
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

    let mut all_blocks = Vec::<TemplateBlock>::new();
    TemplateBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<TemplateBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
