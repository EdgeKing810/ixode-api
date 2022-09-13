use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::{
    sub_condition::Condition, sub_object_pair::ObjectPair,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub pairs: Vec<ObjectPair>,
    pub conditions: Vec<Condition>,
}

impl ReturnBlock {
    pub fn create(all_blocks: &mut Vec<ReturnBlock>, global_index: u32, block_index: u32) {
        let new_block = ReturnBlock {
            global_index: global_index,
            block_index: block_index,
            pairs: vec![],
            conditions: vec![],
        };
        all_blocks.push(new_block);
    }

    pub fn exist(all_blocks: &Vec<ReturnBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn add_pair(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        new_pair: ObjectPair,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.pairs.push(new_pair);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn remove_pair(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        pair_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_pairs = Vec::<ObjectPair>::new();
                if pair_index >= block.pairs.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of pairs present"),
                    ));
                }

                for n in 0..block.pairs.len() {
                    if n as u32 != pair_index {
                        updated_pairs.push(block.pairs[n].clone());
                    }
                }

                block.pairs = updated_pairs;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn set_pairs(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        pairs: Vec<ObjectPair>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.pairs = pairs;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_conditions = Vec::<Condition>::new();
                if condition_index >= block.conditions.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of conditions present"),
                    ));
                }

                for n in 0..block.conditions.len() {
                    if n as u32 != condition_index {
                        updated_conditions.push(block.conditions[n].clone());
                    }
                }

                block.conditions = updated_conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<ReturnBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ReturnBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Return Block not found")));
        }

        let updated_blocks: Vec<ReturnBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| ReturnBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                pairs: block.pairs.clone(),
                conditions: block.conditions.clone(),
            })
            .collect::<Vec<ReturnBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<ReturnBlock>) -> String {
        let mut stringified_blocks = String::new();

        for block in all_blocks {
            stringified_blocks = format!(
                "{}{}{}",
                stringified_blocks,
                if stringified_blocks.chars().count() > 1 {
                    "\n"
                } else {
                    ""
                },
                ReturnBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<ReturnBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("RETURN (").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 1")));
        }

        current_block = current_block[1].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 2")));
        }

        current_block = current_block[0].split(",").collect::<Vec<&str>>();
        if current_block.len() < 2 {
            return Err((500, String::from("Error: Invalid block_str string / 3")));
        }

        let global_index = match current_block[0].trim().parse::<u32>() {
            Ok(idx) => idx,
            Err(e) => return Err((500, format!("Error: Invalid block_str string / 4: {}", e))),
        };

        let block_index = match current_block[1].trim().parse::<u32>() {
            Ok(idx) => idx,
            Err(e) => return Err((500, format!("Error: Invalid block_str string / 5: {}", e))),
        };

        current_block = block_str.split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 6")));
        }

        let current_block_tmp = current_block[1..].join(")");
        current_block = current_block_tmp
            .split("conditions=")
            .collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 7")));
        }

        let mut all_pairs: Vec<ObjectPair> = Vec::new();
        let pairs_list = current_block[0].trim().split(">").collect::<Vec<&str>>();

        for p_str in pairs_list {
            if p_str.len() < 1 {
                continue;
            }

            if let Err(e) = ObjectPair::from_string(&mut all_pairs, p_str) {
                return Err((500, format!("Error: Invalid block_str string / 8: {}", e.1)));
            };
        }

        current_block = block_str.split("conditions=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 9")));
        }

        let mut all_conditions: Vec<Condition> = Vec::new();
        let conditions_list = current_block[1].trim().split(">").collect::<Vec<&str>>();

        for c_str in conditions_list {
            if c_str.len() < 1 {
                continue;
            }

            if let Err(e) = Condition::from_string(&mut all_conditions, c_str) {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 10: {}", e.1),
                ));
            };
        }

        ReturnBlock::create(all_blocks, global_index, block_index);

        match ReturnBlock::set_pairs(all_blocks, global_index, all_pairs) {
            Ok(f) => f,
            Err(e) => {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 12: {}", e.1),
                ))
            }
        };

        match ReturnBlock::set_conditions(all_blocks, global_index, all_conditions) {
            Ok(_) => Ok(()),
            Err(e) => Err((
                500,
                format!("Error: Invalid block_str string / 13: {}", e.1),
            )),
        }
    }

    pub fn to_string(block: ReturnBlock) -> String {
        let pairs_str = ObjectPair::stringify(&block.pairs);
        let conditions_str = Condition::stringify(&block.conditions);

        format!(
            "RETURN ({},{}) {} conditions={}",
            block.global_index, block.block_index, pairs_str, conditions_str
        )
    }
}
