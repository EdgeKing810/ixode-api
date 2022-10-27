use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::sub_object_pair::ObjectPair;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub pairs: Vec<ObjectPair>,
}

impl ObjectBlock {
    pub fn create(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = ObjectBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            pairs: vec![],
        };
        all_blocks.push(new_block);

        if !has_error {
            let local_name_update = Self::update_local_name(all_blocks, global_index, local_name);
            if let Err(e) = local_name_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_block = Self::delete(all_blocks, global_index);
            if let Err(e) = delete_block {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_blocks: &Vec<ObjectBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_blocks: &Vec<ObjectBlock>, global_index: u32) -> Option<ObjectBlock> {
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                return Some(block.clone());
            }
        }

        None
    }

    pub fn update_local_name(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ObjectBlock> = None;

        if !String::from(local_name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: local_name contains an invalid character"),
            ));
        }

        if String::from(local_name.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: local_name does not contain enough characters"),
            ));
        } else if String::from(local_name.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: local_name contains too many characters"),
            ));
        }

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.local_name = local_name.trim().to_string();
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Object Block not found")));
        }

        Ok(())
    }

    pub fn add_pair(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
        new_pair: ObjectPair,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ObjectBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.pairs.push(new_pair);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Object Block not found")));
        }

        Ok(())
    }

    pub fn remove_pair(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
        pair_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ObjectBlock> = None;

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
            return Err((404, String::from("Error: Object Block not found")));
        }

        Ok(())
    }

    pub fn set_pairs(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
        pairs: Vec<ObjectPair>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ObjectBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.pairs = pairs;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Object Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<ObjectBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ObjectBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Object Block not found")));
        }

        let updated_blocks: Vec<ObjectBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| ObjectBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                pairs: block.pairs.clone(),
            })
            .collect::<Vec<ObjectBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<ObjectBlock>) -> String {
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
                ObjectBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<ObjectBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("OBJECT (").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at start of indexes declaration")));
        }

        current_block = current_block[1].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at end of indexes declaration")));
        }

        current_block = current_block[0].split(",").collect::<Vec<&str>>();
        if current_block.len() < 2 {
            return Err((500, String::from("in format of indexes declaration")));
        }

        let global_index = match current_block[0].trim().parse::<u32>() {
            Ok(idx) => idx,
            Err(e) => return Err((500, format!("at global_index -> {}", e))),
        };

        let block_index = match current_block[1].trim().parse::<u32>() {
            Ok(idx) => idx,
            Err(e) => return Err((500, format!("at local_index -> {}", e))),
        };

        current_block = block_str.split("[").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at start of local_name declaration")));
        }

        current_block = current_block[1].split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at end of local_name declaration")));
        }

        let local_name = current_block[0];

        let mut all_pairs: Vec<ObjectPair> = Vec::new();

        current_block = block_str.split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at start of object_pairs declaration")));
        }
        let pairs_list_tmp = current_block[1..].join("]");

        let pairs_list = pairs_list_tmp.trim().split(">").collect::<Vec<&str>>();

        for p_str in pairs_list {
            if p_str.len() < 1 {
                continue;
            }

            if let Err(e) = ObjectPair::from_string(&mut all_pairs, p_str) {
                return Err((500, format!("while processing object_pair -> {}", e.1)));
            };
        }

        match ObjectBlock::create(all_blocks, global_index, block_index, local_name) {
            Ok(f) => f,
            Err(e) => return Err((500, format!("while processing block -> {}", e.1))),
        };

        match ObjectBlock::set_pairs(all_blocks, global_index, all_pairs) {
            Ok(_) => Ok(()),
            Err(e) => Err((500, format!("while processing block -> {}", e.1))),
        }
    }

    pub fn to_string(block: ObjectBlock) -> String {
        let pairs_str = ObjectPair::stringify(&block.pairs);

        format!(
            "OBJECT ({},{}) [{}] {}",
            block.global_index, block.block_index, block.local_name, pairs_str,
        )
    }
}
