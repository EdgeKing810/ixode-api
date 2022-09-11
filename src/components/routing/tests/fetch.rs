#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::blocks::fetch_block::FetchBlock;

#[test]
pub fn run_routing_fetch_one() {
    println!("---> Running Routing Fetcher One");
    // FETCH (2,1) [users,users]

    let mut all_blocks = Vec::<FetchBlock>::new();
    if let Err(e) = FetchBlock::create(&mut all_blocks, 2, 1, "users", "users") {
        println!("Error: {:#?}", e);
        return;
    }

    println!("{}", FetchBlock::to_string(all_blocks[0].clone()));
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

    let block_str = "FETCH (2,1) [users,users]";

    let mut all_blocks = Vec::<FetchBlock>::new();
    FetchBlock::from_string(&mut all_blocks, block_str).unwrap();

    println!("{:#?}", all_blocks[0].clone());
}
