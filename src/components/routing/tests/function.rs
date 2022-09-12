#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::function_block::FunctionBlock;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_function::Function;
#[allow(unused_imports)]
use crate::components::routing::submodules::sub_ref_data::RefData;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::function_block::FunctionBlock>,
) {
    let func = Function::create("V4");

    if let Err(e) = crate::components::routing::blocks::function_block::FunctionBlock::create(
        all_blocks,
        44,
        9,
        "notificationID",
        func,
    ) {
        println!("Error: {:#?}", e);
        return;
    }
}

fn get_block_str_one() -> String {
    "FUNCTION (44,9) [notificationID] {V4=}".to_string()
}

fn make_block_two(
    all_blocks: &mut Vec<crate::components::routing::blocks::function_block::FunctionBlock>,
) {
    let mut func = Function::create("PAGINATE");
    let params = vec![
        RefData::create(false, "INTEGER", "0").unwrap(),
        RefData::create(false, "INTEGER", "20").unwrap(),
    ];
    Function::set_params(&mut func, params);

    if let Err(e) = crate::components::routing::blocks::function_block::FunctionBlock::create(
        all_blocks,
        44,
        9,
        "notificationID",
        func,
    ) {
        println!("Error: {:#?}", e);
        return;
    }
}

fn get_block_str_two() -> String {
    "FUNCTION (44,9) [notificationID] {PAGINATE=[,INTEGER,0]>[,INTEGER,20]}".to_string()
}

#[test]
pub fn run_routing_function_one() {
    println!("---> Running Routing Function One");
    // FUNCTION (44,9) [notificationID] {V4=}

    let mut all_blocks = Vec::<FunctionBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        FunctionBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_function_two() {
    println!("---> Running Routing Function Two");

    // FunctionBlock {
    //     global_index: 44,
    //     block_index: 9,
    //     local_name: "notificationID",
    //     func: Function {
    //         id: V4,
    //         params: []
    //     }
    // }

    let mut all_blocks = Vec::<FunctionBlock>::new();
    FunctionBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<FunctionBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}

#[test]
pub fn run_routing_function_three() {
    println!("---> Running Routing Function Three");
    // FUNCTION (44,9) [notificationID] {PAGINATE=[,INTEGER,0]>[,INTEGER,20]}

    let mut all_blocks = Vec::<FunctionBlock>::new();
    make_block_two(&mut all_blocks);

    assert_eq!(
        get_block_str_two(),
        FunctionBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_function_four() {
    println!("---> Running Routing Function Four");

    // FunctionBlock {
    //     global_index: 44,
    //     block_index: 9,
    //     local_name: "notificationID",
    //     func: Function {
    //         id: PAGINATE,
    //         params: [
    //             RefData {
    //                 ref_var: false,
    //                 rtype: INTEGER,
    //                 data: "0"
    //             },
    //             RefData {
    //                 ref_var: false,
    //                 rtype: INTEGER,
    //                 data: "20"
    //             }
    //         ]
    //     }
    // }

    let mut all_blocks = Vec::<FunctionBlock>::new();
    FunctionBlock::from_string(&mut all_blocks, &get_block_str_two()).unwrap();

    let mut all_blocks_duplicate = Vec::<FunctionBlock>::new();
    make_block_two(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
