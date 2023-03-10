use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::{
        constraint_property::ConstraintProperty,
        routing::submodules::{sub_condition::Condition, sub_operation::Operation},
    },
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignmentBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub conditions: Vec<Condition>,
    pub operations: Vec<Operation>,
}

impl AssignmentBlock {
    pub fn create(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = AssignmentBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            conditions: vec![],
            operations: vec![],
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

    pub fn exist(all_blocks: &Vec<AssignmentBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_blocks: &Vec<AssignmentBlock>, global_index: u32) -> Option<AssignmentBlock> {
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                return Some(block.clone());
            }
        }

        None
    }

    pub fn update_local_name(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "assignment_block",
            "local_name",
            local_name,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.local_name = final_value;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

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
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn add_operation(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        new_operation: Operation,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.operations.push(new_operation);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn remove_operation(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        operation_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_operations = Vec::<Operation>::new();
                if operation_index >= block.operations.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of operations present"),
                    ));
                }

                for n in 0..block.operations.len() {
                    if n as u32 != operation_index {
                        updated_operations.push(block.operations[n].clone());
                    }
                }

                block.operations = updated_operations;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn set_operations(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
        operations: Vec<Operation>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.operations = operations;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<AssignmentBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<AssignmentBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Assignment Block not found")));
        }

        let updated_blocks: Vec<AssignmentBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| AssignmentBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                conditions: block.conditions.clone(),
                operations: block.operations.clone(),
            })
            .collect::<Vec<AssignmentBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<AssignmentBlock>) -> String {
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
                AssignmentBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<AssignmentBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("ASSIGN (").collect::<Vec<&str>>();
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

        let mut all_conditions: Vec<Condition> = Vec::new();
        let mut conditions_list = block_str.split("}").collect::<Vec<&str>>();
        if conditions_list.len() > 1 {
            let conditions_list_str = conditions_list[1].trim();
            conditions_list = conditions_list_str.split(">").collect::<Vec<&str>>();

            for c_str in conditions_list {
                if c_str.len() < 1 {
                    continue;
                }

                if let Err(e) = Condition::from_string(&mut all_conditions, c_str) {
                    return Err((500, format!("at condition -> {}", e.1)));
                };
            }
        }

        let mut all_operations: Vec<Operation> = Vec::new();
        let mut operations_list = block_str.split("{").collect::<Vec<&str>>();
        if operations_list.len() <= 1 {
            return Err((500, String::from("at start of operations declaration")));
        }

        operations_list = operations_list[1].split("}").collect::<Vec<&str>>();
        if operations_list.len() <= 1 {
            return Err((500, String::from("at end of operations declaration")));
        }

        let operations_list_str = operations_list[0].trim();
        operations_list = operations_list_str.split(">").collect::<Vec<&str>>();

        for o_str in operations_list {
            if o_str.len() < 1 {
                continue;
            }

            if let Err(e) = Operation::from_string(&mut all_operations, o_str) {
                return Err((500, format!("at operation -> {}", e.1)));
            };
        }

        if let Err(e) = AssignmentBlock::create(all_blocks, global_index, block_index, local_name) {
            return Err((500, format!("at processing -> {}", e.1)));
        };

        if let Err(e) = AssignmentBlock::set_conditions(all_blocks, global_index, all_conditions) {
            return Err((500, format!("at processing -> {}", e.1)));
        }

        match AssignmentBlock::set_operations(all_blocks, global_index, all_operations) {
            Ok(_) => Ok(()),
            Err(e) => Err((500, format!("at setting operations -> {}", e.1))),
        }
    }

    pub fn to_string(block: AssignmentBlock) -> String {
        let conditions_str = Condition::stringify(&block.conditions);
        let operations_str = Operation::stringify(&block.operations);

        format!(
            "ASSIGN ({},{}) [{}] {{{}}} {}",
            block.global_index, block.block_index, block.local_name, operations_str, conditions_str
        )
    }
}
