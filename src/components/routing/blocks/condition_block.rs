use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::{
    sub_condition::Condition, sub_condition_action::ConditionAction, sub_fail_obj::FailObj,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConditionBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub conditions: Vec<Condition>,
    pub action: ConditionAction,
    pub fail: FailObj,
}

impl ConditionBlock {
    pub fn create(
        all_blocks: &mut Vec<ConditionBlock>,
        global_index: u32,
        block_index: u32,
        action: &str,
        fail_status: u32,
        fail_message: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let fail_obj = match FailObj::create(fail_status, fail_message) {
            Ok(fail_obj) => fail_obj,
            Err(e) => {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
                FailObj::default()
            }
        };

        let new_block = ConditionBlock {
            global_index: global_index,
            block_index: block_index,
            conditions: vec![],
            action: ConditionAction::from(action),
            fail: fail_obj,
        };
        all_blocks.push(new_block);

        if has_error {
            let delete_block = Self::delete(all_blocks, block_index);
            if let Err(e) = delete_block {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_blocks: &Vec<ConditionBlock>, block_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.block_index == block_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_action(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
        action: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
                found_block = Some(block.clone());
                block.action = ConditionAction::from(action);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Condition Block not found")));
        }

        Ok(())
    }

    pub fn update_fail(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
        fail_status: u32,
        fail_message: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        let fail_obj = match FailObj::create(fail_status, fail_message) {
            Ok(fail_obj) => fail_obj,
            Err(e) => {
                return Err(e);
            }
        };

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
                found_block = Some(block.clone());
                block.fail = fail_obj;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Condition Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Condition Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
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
            return Err((404, String::from("Error: Condition Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Condition Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<ConditionBlock>,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<ConditionBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.block_index == block_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Condition Block not found")));
        }

        let updated_blocks: Vec<ConditionBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.block_index != block_index)
            .map(|block| ConditionBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                conditions: block.conditions.clone(),
                action: block.action.clone(),
                fail: block.fail.clone(),
            })
            .collect::<Vec<ConditionBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<ConditionBlock>) -> String {
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
                ConditionBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<ConditionBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("CONDITION (").collect::<Vec<&str>>();
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

        let block_index = match current_block[0].trim().parse::<u32>() {
            Ok(idx) => idx,
            Err(e) => return Err((500, format!("Error: Invalid block_str string / 4: {}", e))),
        };

        let global_index = match current_block[1].trim().parse::<u32>() {
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

        let action_str = current_block[0];

        current_block = block_str.split("[").collect::<Vec<&str>>();
        if current_block.len() <= 2 {
            return Err((500, String::from("Error: Invalid block_str string / 8")));
        }

        current_block = current_block[2].split("]").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 9")));
        }

        let fail_obj = match FailObj::from_string(&format!("[{}]", current_block[0])) {
            Ok(fail_obj) => fail_obj,
            Err(e) => {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 10: {}", e.1),
                ))
            }
        };

        let mut conditions_list = block_str.split("([").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 11")));
        }

        let conditions_list_str = format!("([{}", conditions_list[1..].join("(["));
        conditions_list = conditions_list_str.split(">").collect::<Vec<&str>>();

        let mut all_conditions: Vec<Condition> = Vec::new();
        for c_str in conditions_list {
            if let Err(e) = Condition::from_string(&mut all_conditions, c_str) {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 12: {}", e.1),
                ));
            };
        }

        if let Err(e) = ConditionBlock::create(
            all_blocks,
            global_index,
            block_index,
            action_str,
            fail_obj.status,
            &fail_obj.message,
        ) {
            return Err(e);
        };

        return ConditionBlock::set_conditions(all_blocks, block_index, all_conditions);
    }

    pub fn to_string(block: ConditionBlock) -> String {
        let conditions_str = Condition::stringify(&block.conditions);
        format!(
            "CONDITION ({},{}) [{}] {} {}",
            block.block_index,
            block.global_index,
            ConditionAction::to(block.action),
            FailObj::to_string(block.fail),
            conditions_str
        )
    }
}
