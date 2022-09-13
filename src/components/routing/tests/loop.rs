#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::loop_block::LoopBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(all_blocks: &mut Vec<crate::components::routing::blocks::loop_block::LoopBlock>) {
    let min = RefData::create(false, "INTEGER", "0").unwrap();
    let max = RefData::create(true, "INTEGER", "currentProfilesLength").unwrap();

    if let Err(e) = crate::components::routing::blocks::loop_block::LoopBlock::create(
        all_blocks, 5, 1, "x", min, max,
    ) {
        println!("Error: {:#?}", e);
        return;
    }
}

fn get_block_str_one() -> String {
    "LOOP (5,1) [x] ([,INTEGER,0]|[ref,INTEGER,currentProfilesLength])".to_string()
}

#[test]
pub fn run_routing_loop_one() {
    println!("---> Running Routing Loop One");
    // LOOP (5,1) [x] ([,INTEGER,0]|[ref,INTEGER,currentProfilesLength])

    let mut all_blocks = Vec::<LoopBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        LoopBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_loop_two() {
    println!("---> Running Routing Loop Two");

    // LoopBlock {
    //     global_index: 5,
    //     block_index: 1,
    //     local_name: "x",
    //     min: RefData {
    //         ref_var: false,
    //         rtype: INTEGER,
    //         data: "0"
    //     },
    //     max: RefData {
    //         ref_var: true,
    //         rtype: INTEGER,
    //         data: "currentProfilesLength"
    //     }
    // }

    let mut all_blocks = Vec::<LoopBlock>::new();
    LoopBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<LoopBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
