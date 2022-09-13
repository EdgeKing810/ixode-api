#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::return_block::ReturnBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_object_pair::ObjectPair;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::return_block::ReturnBlock>,
) {
    crate::components::routing::blocks::return_block::ReturnBlock::create(all_blocks, 50, 10);

    let mut all_pairs = Vec::<ObjectPair>::new();

    let mut ref_data = RefData::create(false, "INTEGER", "0").unwrap();
    ObjectPair::create(&mut all_pairs, "error", ref_data).unwrap();

    ref_data = RefData::create(true, "STRING", "responseMessage").unwrap();
    ObjectPair::create(&mut all_pairs, "message", ref_data).unwrap();

    crate::components::routing::blocks::return_block::ReturnBlock::set_pairs(
        all_blocks, 50, all_pairs,
    )
    .unwrap();
}

fn get_block_str_one() -> String {
    "RETURN (50,10) (error=[,INTEGER,0])>(message=[ref,STRING,responseMessage]) conditions="
        .to_string()
}

#[test]
pub fn run_routing_return_one() {
    println!("---> Running Routing Return One");
    // RETURN (50,10) (error=[,INTEGER,0])>(message=[ref,STRING,responseMessage]) conditions=

    let mut all_blocks = Vec::<ReturnBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        ReturnBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_return_two() {
    println!("---> Running Routing Return Two");

    // ReturnBlock {
    //     global_index: 50,
    //     block_index: 10,
    //     pairs: [
    //         ObjectPair {
    //             id: "error",
    //             data: RefData {
    //                 ref_var: false,
    //                 rtype: INTEGER,
    //                 data: "0"
    //             }
    //         },
    //         ObjectPair {
    //             id: "message",
    //             data: RefData {
    //                 ref_var: true,
    //                 rtype: STRING,
    //                 data: "responseMessage"
    //             }
    //         }
    //     ],
    //     conditions: []
    // }

    let mut all_blocks = Vec::<ReturnBlock>::new();
    ReturnBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<ReturnBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
