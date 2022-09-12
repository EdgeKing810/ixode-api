#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::object_block::ObjectBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_object_pair::ObjectPair;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::object_block::ObjectBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::object_block::ObjectBlock::create(
        all_blocks,
        46,
        9,
        "newNotification",
    ) {
        println!("Error: {:#?}", e);
        return;
    }

    let mut all_pairs = Vec::<ObjectPair>::new();

    let mut ref_data = RefData::create(true, "STRING", "profileID").unwrap();
    ObjectPair::create(&mut all_pairs, "uid", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "notificationID").unwrap();
    ObjectPair::create(&mut all_pairs, "notificationID", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "notificationContent").unwrap();
    ObjectPair::create(&mut all_pairs, "content", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "uid").unwrap();
    ObjectPair::create(&mut all_pairs, "profileID", ref_data).unwrap();

    ref_data = RefData::create(false, "STRING", "follow_send").unwrap();
    ObjectPair::create(&mut all_pairs, "type", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "generatedOn").unwrap();
    ObjectPair::create(&mut all_pairs, "created_on", ref_data).unwrap();

    ref_data = RefData::create(false, "BOOLEAN", "false").unwrap();
    ObjectPair::create(&mut all_pairs, "read", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "notificationRedirect").unwrap();
    ObjectPair::create(&mut all_pairs, "redirect", ref_data).unwrap();

    crate::components::routing::blocks::object_block::ObjectBlock::set_pairs(
        all_blocks, 46, all_pairs,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "OBJECT (46,9) [newNotification] (uid=[ref,STRING,profileID])>(notificationID=[ref,STRING,notificationID])>(content=[ref,STRING,notificationContent])>(profileID=[ref,STRING,uid])>(type=[,STRING,follow_send])>(created_on=[ref,STRING,generatedOn])>(read=[,BOOLEAN,false])>(redirect=[ref,STRING,notificationRedirect])".to_string()
}

#[test]
pub fn run_routing_object_one() {
    println!("---> Running Routing Object One");
    // OBJECT (46,9) [newNotification] (uid=[ref,STRING,profileID])>(notificationID=[ref,STRING,notificationID])>(content=[ref,STRING,notificationContent])>(profileID=[ref,STRING,uid])>(type=[,STRING,follow_send])>(created_on=[ref,STRING,generatedOn])>(read=[,BOOLEAN,false])>(redirect=[ref,STRING,notificationRedirect])

    let mut all_blocks = Vec::<ObjectBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        ObjectBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_object_two() {
    println!("---> Running Routing Object Two");

    // ObjectBlock {
    //     global_index: 46,
    //     block_index: 9,
    //     local_name: "newNotification",
    //     pairs: [
    //         ObjectPair {
    //             id: "uid",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "profileID"
    //             }
    //         },
    //         ObjectPair {
    //             id: "notificationID",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "notificationID"
    //             }
    //         },
    //         ObjectPair {
    //             id: "content",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "notificationContent"
    //             }
    //         },
    //         ObjectPair {
    //             id: "profileID",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "uid"
    //             }
    //         },
    //         ObjectPair {
    //             id: "type",
    //             data: RefData {
    //                 ref_var: false,
    //                 rtype: STRING,
    //                 data: "follow_send"
    //             }
    //         },
    //         ObjectPair {
    //             id: "created_on",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "generatedOn"
    //             }
    //         },
    //         ObjectPair {
    //             id: "read",
    //             data: RefData {
    //                 ref_var: false,
    //                 rtype: BOOLEAN,
    //                 data: "false"
    //             }
    //         },
    //         ObjectPair {
    //             id: "redirect",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "notificationRedirect"
    //             }
    //         }
    //     ]
    // }

    let mut all_blocks = Vec::<ObjectBlock>::new();
    ObjectBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<ObjectBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
