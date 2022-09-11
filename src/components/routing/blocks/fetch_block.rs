use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FetchBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub ref_col: String,
}

impl FetchBlock {
    pub fn create(
        all_blocks: &mut Vec<FetchBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
        ref_col: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = FetchBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            ref_col: "".to_string(),
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

        if !has_error {
            let ref_col_update = Self::update_ref_col(all_blocks, global_index, ref_col);
            if let Err(e) = ref_col_update {
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

    pub fn exist(all_blocks: &Vec<FetchBlock>, global_index: u32) -> bool {
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
        all_blocks: &mut Vec<FetchBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FetchBlock> = None;

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
            return Err((404, String::from("Error: Fetch Block not found")));
        }

        Ok(())
    }

    pub fn update_ref_col(
        all_blocks: &mut Vec<FetchBlock>,
        global_index: u32,
        ref_col: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FetchBlock> = None;

        if !String::from(ref_col)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: ref_col contains an invalid character"),
            ));
        }

        if String::from(ref_col.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: ref_col does not contain enough characters"),
            ));
        } else if String::from(ref_col.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: ref_col contains too many characters"),
            ));
        }

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.ref_col = ref_col.trim().to_string();
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Fetch Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<FetchBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FetchBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Fetch Block not found")));
        }

        let updated_blocks: Vec<FetchBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| FetchBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                ref_col: block.ref_col.clone(),
            })
            .collect::<Vec<FetchBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<FetchBlock>) -> String {
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
                FetchBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<FetchBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("FETCH (").collect::<Vec<&str>>();
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

        current_block = block_str.split("[").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 6")));
        }

        current_block = current_block[1].split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 7")));
        }

        current_block = current_block[0].split(",").collect::<Vec<&str>>();
        if current_block.len() < 2 {
            return Err((500, String::from("Error: Invalid block_str string / 8")));
        }

        return Self::create(
            all_blocks,
            global_index,
            block_index,
            current_block[0],
            current_block[1],
        );
    }

    pub fn to_string(block: FetchBlock) -> String {
        format!(
            "FETCH ({},{}) [{},{}]",
            block.global_index, block.block_index, block.local_name, block.ref_col
        )
    }
}
