use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::sub_ref_data::RefData;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub min: RefData,
    pub max: RefData,
}

impl LoopBlock {
    pub fn create(
        all_blocks: &mut Vec<LoopBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
        min: RefData,
        max: RefData,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = LoopBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            min: min,
            max: max,
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

    pub fn exist(all_blocks: &Vec<LoopBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_local_name(
        all_blocks: &mut Vec<LoopBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<LoopBlock> = None;

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
            return Err((404, String::from("Error: Loop Block not found")));
        }

        Ok(())
    }

    pub fn update_min(
        all_blocks: &mut Vec<LoopBlock>,
        global_index: u32,
        min: RefData,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<LoopBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.min = min;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Loop Block not found")));
        }

        Ok(())
    }

    pub fn update_max(
        all_blocks: &mut Vec<LoopBlock>,
        global_index: u32,
        max: RefData,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<LoopBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.max = max;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Loop Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<LoopBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<LoopBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Loop Block not found")));
        }

        let updated_blocks: Vec<LoopBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| LoopBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                min: block.min.clone(),
                max: block.max.clone(),
            })
            .collect::<Vec<LoopBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<LoopBlock>) -> String {
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
                LoopBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<LoopBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("LOOP (").collect::<Vec<&str>>();
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

        current_block = block_str.split("(").collect::<Vec<&str>>();
        if current_block.len() <= 2 {
            return Err((500, String::from("at start of range declaration")));
        }

        current_block = current_block[2].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at end of range declaration")));
        }

        current_block = current_block[0].split("|").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("in format of range declaration")));
        }

        let min = match RefData::from_string(current_block[0]) {
            Ok(m) => m,
            Err(e) => return Err((500, format!("while processing min -> {}", e.1))),
        };

        let max = match RefData::from_string(current_block[1]) {
            Ok(m) => m,
            Err(e) => return Err((500, format!("while processing max -> {}", e.1))),
        };

        match LoopBlock::create(all_blocks, global_index, block_index, local_name, min, max) {
            Ok(_) => Ok(()),
            Err(e) => Err((500, format!("while processing block -> {}", e.1))),
        }
    }

    pub fn to_string(block: LoopBlock) -> String {
        format!(
            "LOOP ({},{}) [{}] ({}|{})",
            block.global_index,
            block.block_index,
            block.local_name,
            RefData::to_string(block.min.clone()),
            RefData::to_string(block.max.clone()),
        )
    }
}
