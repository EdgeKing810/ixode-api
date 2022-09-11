#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::fetch_block::FetchBlock;

fn make_block_one(
    all_blocks: &mut Vec<crate::components::routing::blocks::fetch_block::FetchBlock>,
) {
    if let Err(e) = crate::components::routing::blocks::fetch_block::FetchBlock::create(
        all_blocks, 2, 1, "users", "users",
    ) {
        println!("Error: {:#?}", e);
        return;
    }
}

fn get_block_str_one() -> String {
    "FETCH (2,1) [users,users]".to_string()
}

#[test]
pub fn run_routing_fetch_one() {
    println!("---> Running Routing Fetcher One");
    // FETCH (2,1) [users,users]

    let mut all_blocks = Vec::<FetchBlock>::new();
    make_block_one(&mut all_blocks);

    assert_eq!(
        get_block_str_one(),
        FetchBlock::to_string(all_blocks[0].clone())
    );
}

#[test]
pub fn run_routing_fetch_two() {
    println!("---> Running Routing Fetcher Two");

    // FetchBlock {
    //     global_index: 2,
    //     block_index: 1,
    //     local_name: "users",
    //     ref_col: "users",
    // }

    let mut all_blocks = Vec::<FetchBlock>::new();
    FetchBlock::from_string(&mut all_blocks, &get_block_str_one()).unwrap();

    let mut all_blocks_duplicate = Vec::<FetchBlock>::new();
    make_block_one(&mut all_blocks_duplicate);

    assert_eq!(all_blocks_duplicate[0], all_blocks[0]);
}
