#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::update_block::UpdateBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_condition::Condition;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_filter::Filter;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_update_target::UpdateTarget;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::update_block::UpdateBlock>,
) {
    let add: Option<RefData> = None;
    let set: Option<RefData> =
        Some(RefData::create(true, "OTHER", "targetProfile_followers").unwrap());

    let mut all_filters = Vec::<Filter>::new();
    let mut right = RefData::create(true, "STRING", "uid").unwrap();
    Filter::create(&mut all_filters, right, "NOT_EQUAL_TO", false, "NONE");
    let filter: Option<Filter> = Some(all_filters[0].clone());

    if let Err(e) = crate::components::routing::blocks::update_block::UpdateBlock::create(
        all_blocks,
        31,
        7,
        "profiles",
        "followers",
        true,
        add,
        set,
        filter,
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_conditions = Vec::<Condition>::new();
    let left = RefData::create(true, "BOOLEAN", "status").unwrap();
    right = RefData::create(false, "BOOLEAN", "false").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "NONE");

    let mut all_targets = Vec::<UpdateTarget>::new();
    UpdateTarget::create(&mut all_targets, "uid").unwrap();

    all_filters = Vec::<Filter>::new();
    right = RefData::create(true, "STRING", "profileID").unwrap();
    Filter::create(&mut all_filters, right, "EQUAL_TO", false, "NONE");
    UpdateTarget::set_conditions(&mut all_targets, 0, all_filters).unwrap();

    crate::components::routing::blocks::update_block::UpdateBlock::set_conditions(
        all_blocks,
        31,
        all_conditions,
    )
    .unwrap();

    crate::components::routing::blocks::update_block::UpdateBlock::set_targets(
        all_blocks,
        31,
        all_targets,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "UPDATE (31,7) [profiles,followers,true] {uid|([ref,STRING,profileID]|EQUAL_TO|not=false|next=NONE)} (add=) (set=[ref,OTHER,targetProfile_followers]) {filter=([ref,STRING,uid]|NOT_EQUAL_TO|not=false|next=NONE)} conditions=([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,false]|not=false|next=NONE)".to_string()
}

fn make_block_two(
    all_blocks: &mut Vec<crate::components::routing::blocks::update_block::UpdateBlock>,
) {
    let add: Option<RefData> = Some(RefData::create(true, "STRING", "profileID").unwrap());
    let set: Option<RefData> =
        Some(RefData::create(true, "OTHER", "currentProfile_sent_follow_requests").unwrap());

    let filter: Option<Filter> = None;

    if let Err(e) = crate::components::routing::blocks::update_block::UpdateBlock::create(
        all_blocks,
        32,
        8,
        "profiles",
        "sent_follow_requests",
        false,
        add,
        set,
        filter,
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_conditions = Vec::<Condition>::new();
    let mut left = RefData::create(true, "BOOLEAN", "status").unwrap();
    let mut right = RefData::create(false, "BOOLEAN", "true").unwrap();
    Condition::create(&mut all_conditions, left, right, "EQUAL_TO", false, "AND");
    left = RefData::create(true, "STRING", "targetProfile_account_type").unwrap();
    right = RefData::create(false, "STRING", "public").unwrap();
    Condition::create(
        &mut all_conditions,
        left,
        right,
        "NOT_EQUAL_TO",
        false,
        "NONE",
    );

    let mut all_targets = Vec::<UpdateTarget>::new();
    UpdateTarget::create(&mut all_targets, "uid").unwrap();

    let mut all_filters = Vec::<Filter>::new();
    right = RefData::create(true, "STRING", "uid").unwrap();
    Filter::create(&mut all_filters, right, "EQUAL_TO", false, "NONE");
    UpdateTarget::set_conditions(&mut all_targets, 0, all_filters).unwrap();

    crate::components::routing::blocks::update_block::UpdateBlock::set_conditions(
        all_blocks,
        32,
        all_conditions,
    )
    .unwrap();

    crate::components::routing::blocks::update_block::UpdateBlock::set_targets(
        all_blocks,
        32,
        all_targets,
    )
    .unwrap();
}

fn get_block_str_two() -> String {
    "UPDATE (32,8) [profiles,sent_follow_requests,false] {uid|([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)} (add=[ref,STRING,profileID]) (set=[ref,OTHER,currentProfile_sent_follow_requests]) {filter=} conditions=([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=AND)>([ref,STRING,targetProfile_account_type]|NOT_EQUAL_TO|[,STRING,public]|not=false|next=NONE)".to_string()
}

#[test]
pub fn run_routing_update_one() {
    println!("---> Running Routing Update One");
    // UPDATE (31,7) [profiles,followers,true] {uid|([ref,STRING,profileID]|EQUAL_TO|not=false|next=NONE)} (add=) (set=[ref,OTHER,targetProfile_followers]) {filter=([ref,STRING,uid]|NOT_EQUAL_TO|not=false|next=NONE)} conditions=([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,false]|not=false|next=NONE)

    let mut all_blocks = Vec::<UpdateBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        UpdateBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_update_two() {
    println!("---> Running Routing Update Two");

    // UpdateBlock {
    //     global_index: 31,
    //     block_index: 7,
    //     ref_col: "profiles",
    //     ref_property: "followers",
    //     save: true,
    //     targets: [
    //         UpdateTarget {
    //             field: "uid",
    //             conditions: [
    //                 Filter {
    //                     right: RefData {
    //                         ref_var: true,
    //                         rtype: STRING,
    //                         data: "profileID"
    //                     },
    //                     operation_type: EQUAL_TO,
    //                     not: false,
    //                     next: NONE
    //                 }
    //             ]
    //         }
    //     ],
    //     add: None,
    //     set: Some(
    //         RefData {
    //             ref_var: true,
    //             rtype: OTHER,
    //             data: "targetProfile_followers"
    //         }
    //     ),
    //     filter: Some(
    //         Filter {
    //             right: RefData {
    //                  ref_var: true,
    //                  rtype: STRING,
    //                  data: "uid"
    //             },
    //             operation_type: NOT_EQUAL_TO,
    //             not: false,
    //             next: NONE
    //         }
    //     ),
    //     conditions: [
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: BOOLEAN,
    //                 data: "status"
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: BOOLEAN,
    //                 data: "false"
    //             },
    //             condition_type: EQUAL_TO,
    //             not: false,
    //             next: NONE
    //         }
    //     ]
    // }

    let mut all_blocks = Vec::<UpdateBlock>::new();
    UpdateBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<UpdateBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}

#[test]
pub fn run_routing_update_three() {
    println!("---> Running Routing Update Three");
    // UPDATE (32,8) [profiles,sent_follow_requests,false] {uid|([ref,STRING,uid]|EQUAL_TO|not=false|next=NONE)} (add=[ref,STRING,profileID]) (set=[ref,OTHER,currentProfile_sent_follow_requests]) {filter=} conditions=([ref,BOOLEAN,status]|EQUAL_TO|[,BOOLEAN,true]|not=false|next=AND)>([ref,STRING,targetProfile_account_type]|NOT_EQUAL_TO|[,STRING,public]|not=false|next=NONE)

    let mut all_blocks = Vec::<UpdateBlock>::new();
    make_block_two(&mut all_blocks);

    assert_eq!(
        get_block_str_two(),
        UpdateBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_update_four() {
    println!("---> Running Routing Update Four");

    // UpdateBlock {
    //     global_index: 32,
    //     block_index: 8,
    //     ref_col: "profiles",
    //     ref_property: "sent_follow_requests",
    //     save: false,
    //     targets: [
    //         UpdateTarget {
    //             field: "uid",
    //             conditions: [
    //                 Filter {
    //                     right: RefData {
    //                         ref_var: true,
    //                         rtype: STRING,
    //                         data: "uid"
    //                     },
    //                     operation_type: EQUAL_TO,
    //                     not: false,
    //                     next: NONE
    //                 }
    //             ]
    //         }
    //     ],
    //     add: Some(
    //         RefData {
    //             ref_var: true,
    //             rtype: STRING,
    //             data: "profileID""
    //         }
    //     ),
    //     set: Some(
    //         RefData {
    //             ref_var: true,
    //             rtype: OTHER,
    //             data: "currentProfile_sent_follow_requests"
    //         }
    //     ),
    //     filter: None,
    //     conditions: [
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
    //             next: AND
    //         },
    //         Condition {
    //             left: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "targetProfile_account_type"
    //             },
    //             right: RefData {
    //                 ref_var: false,
    //                 rtype: STRING,
    //                 data: "public"
    //             },
    //             condition_type: NOT_EQUAL_TO,
    //             not: false,
    //             next: NONE
    //         }
    //     ]
    // }

    let mut all_blocks = Vec::<UpdateBlock>::new();
    UpdateBlock::from_string(&mut all_blocks, &get_block_str_two()).unwrap();

    let mut all_blocks_duplicate = Vec::<UpdateBlock>::new();
    make_block_two(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
