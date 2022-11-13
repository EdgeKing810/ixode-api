use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::{
        constraint_property::ConstraintProperty, routing::submodules::sub_condition::Condition,
    },
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub ref_col: String,
    pub ref_object: String,
    pub save: bool,
    pub conditions: Vec<Condition>,
}

impl CreateBlock {
    pub fn create(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        block_index: u32,
        ref_col: &str,
        ref_object: &str,
        save: bool,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = CreateBlock {
            global_index: global_index,
            block_index: block_index,
            ref_col: "".to_string(),
            ref_object: "".to_string(),
            save: save,
            conditions: vec![],
        };
        all_blocks.push(new_block);

        if !has_error {
            let ref_col_update = Self::update_ref_col(all_blocks, global_index, ref_col);
            if let Err(e) = ref_col_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let ref_object_update = Self::update_ref_object(all_blocks, global_index, ref_object);
            if let Err(e) = ref_object_update {
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

    pub fn exist(all_blocks: &Vec<CreateBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_blocks: &Vec<CreateBlock>, global_index: u32) -> Option<CreateBlock> {
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                return Some(block.clone());
            }
        }

        None
    }

    pub fn update_ref_col(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        ref_col: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "create_block",
            "ref_col",
            ref_col,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.ref_col = final_value;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn update_ref_object(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        ref_object: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "create_block",
            "ref_object",
            ref_object,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.ref_object = final_value;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn update_save(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        save: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.save = save;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

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
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<CreateBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<CreateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Create Block not found")));
        }

        let updated_blocks: Vec<CreateBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| CreateBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                ref_col: block.ref_col.clone(),
                ref_object: block.ref_object.clone(),
                save: block.save,
                conditions: block.conditions.clone(),
            })
            .collect::<Vec<CreateBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<CreateBlock>) -> String {
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
                CreateBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<CreateBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("CREATE (").collect::<Vec<&str>>();
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
            return Err((500, String::from("at start of ref declaration")));
        }

        current_block = current_block[1].split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at end of ref declaration")));
        }

        current_block = current_block[0].split(",").collect::<Vec<&str>>();
        if current_block.len() < 3 {
            return Err((500, String::from("in format of ref declaration")));
        }

        let ref_col = current_block[0];
        let ref_object = current_block[1];
        let save = current_block[2] == "true";

        let mut all_conditions: Vec<Condition> = Vec::new();

        current_block = block_str.split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("at start of conditions declaration")));
        }
        let conditions_list_tmp = current_block[1..].join("]");

        let conditions_list = conditions_list_tmp.trim().split(">").collect::<Vec<&str>>();

        for c_str in conditions_list {
            if c_str.len() < 1 {
                continue;
            }

            if let Err(e) = Condition::from_string(&mut all_conditions, c_str) {
                return Err((500, format!("while processing condition -> {}", e.1)));
            };
        }

        match CreateBlock::create(
            all_blocks,
            global_index,
            block_index,
            ref_col,
            ref_object,
            save,
        ) {
            Ok(f) => f,
            Err(e) => return Err((500, format!("while processing block -> {}", e.1))),
        };

        match CreateBlock::set_conditions(all_blocks, global_index, all_conditions) {
            Ok(_) => Ok(()),
            Err(e) => Err((500, format!("while processing block -> {}", e.1))),
        }
    }

    pub fn to_string(block: CreateBlock) -> String {
        let conditions_str = Condition::stringify(&block.conditions);

        format!(
            "CREATE ({},{}) [{},{},{}] {}",
            block.global_index,
            block.block_index,
            block.ref_col,
            block.ref_object,
            if block.save { "true" } else { "false" },
            conditions_str,
        )
    }
}
