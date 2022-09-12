use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::{
    sub_condition::Condition, sub_filter::Filter, sub_ref_data::RefData,
    sub_update_target::UpdateTarget,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub ref_col: String,
    pub ref_property: String,
    pub save: bool,
    pub targets: Vec<UpdateTarget>,
    pub add: Option<RefData>,
    pub set: Option<RefData>,
    pub filter: Option<Filter>,
    pub conditions: Vec<Condition>,
}

impl UpdateBlock {
    pub fn create(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        block_index: u32,
        ref_col: &str,
        ref_property: &str,
        save: bool,
        add: Option<RefData>,
        set: Option<RefData>,
        filter: Option<Filter>,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = UpdateBlock {
            global_index: global_index,
            block_index: block_index,
            ref_col: "".to_string(),
            ref_property: "".to_string(),
            save: false,
            targets: vec![],
            add: None,
            set: None,
            filter: None,
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
            let ref_property_update =
                Self::update_ref_property(all_blocks, global_index, ref_property);
            if let Err(e) = ref_property_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let save_update = Self::update_save(all_blocks, global_index, save);
            if let Err(e) = save_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let add_update = Self::update_add(all_blocks, global_index, add);
            if let Err(e) = add_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let set_update = Self::update_set(all_blocks, global_index, set);
            if let Err(e) = set_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let filter_update = Self::update_filter(all_blocks, global_index, filter);
            if let Err(e) = filter_update {
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

    pub fn exist(all_blocks: &Vec<UpdateBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_ref_col(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        ref_col: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

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
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn update_ref_property(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        ref_property: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        if !String::from(ref_property)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: ref_property contains an invalid character"),
            ));
        }

        if String::from(ref_property.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: ref_property contains too many characters"),
            ));
        }

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.ref_property = ref_property.trim().to_string();
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn update_save(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        save: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.save = save;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn update_add(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        add: Option<RefData>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.add = add;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn update_set(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        set: Option<RefData>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.set = set;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn update_filter(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        filter: Option<Filter>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.filter = filter;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn add_target(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        new_target: UpdateTarget,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.targets.push(new_target);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn remove_target(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        target_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_targets = Vec::<UpdateTarget>::new();
                if target_index >= block.targets.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of targets present"),
                    ));
                }

                for n in 0..block.targets.len() {
                    if n as u32 != target_index {
                        updated_targets.push(block.targets[n].clone());
                    }
                }

                block.targets = updated_targets;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn set_targets(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        targets: Vec<UpdateTarget>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.targets = targets;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

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
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<UpdateBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<UpdateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Update Block not found")));
        }

        let updated_blocks: Vec<UpdateBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| UpdateBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                ref_col: block.ref_col.clone(),
                ref_property: block.ref_property.clone(),
                save: block.save,
                targets: block.targets.clone(),
                add: block.add.clone(),
                set: block.set.clone(),
                filter: block.filter.clone(),
                conditions: block.conditions.clone(),
            })
            .collect::<Vec<UpdateBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<UpdateBlock>) -> String {
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
                UpdateBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<UpdateBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("UPDATE (").collect::<Vec<&str>>();
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
        if current_block.len() < 3 {
            return Err((500, String::from("Error: Invalid block_str string / 8")));
        }

        let ref_col = current_block[0];
        let ref_property = current_block[1];
        let save = current_block[2] == "true";

        current_block = block_str.split("(add=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 9")));
        }

        current_block = current_block[1].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 10")));
        }

        let mut add: Option<RefData> = None;
        if current_block[0].trim().len() > 0 {
            match RefData::from_string(current_block[0]) {
                Ok(ref_data) => add = Some(ref_data),
                Err(e) => return Err(e),
            }
        }

        current_block = block_str.split("(set=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 11")));
        }

        current_block = current_block[1].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 12")));
        }

        let mut set: Option<RefData> = None;
        if current_block[0].trim().len() > 0 {
            match RefData::from_string(current_block[0]) {
                Ok(ref_data) => set = Some(ref_data),
                Err(e) => return Err(e),
            }
        }

        current_block = block_str.split("{filter=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 13")));
        }

        current_block = current_block[1].split("}").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 14")));
        }

        let mut all_filters = Vec::<Filter>::new();
        let mut filter: Option<Filter> = None;
        if current_block[0].trim().len() > 0 {
            match Filter::from_string(&mut all_filters, current_block[0]) {
                Ok(_) => filter = Some(all_filters[0].clone()),
                Err(e) => return Err(e),
            }
        }

        current_block = block_str.split("] ").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 15")));
        }

        let current_block_tmp = current_block[1..].join("] ");
        current_block = current_block_tmp.split(" (add=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 16")));
        }

        let mut all_targets: Vec<UpdateTarget> = Vec::new();
        let targets_list = current_block[0].split("%").collect::<Vec<&str>>();
        for target_str in targets_list {
            if let Err(e) = UpdateTarget::from_string(&mut all_targets, target_str) {
                return Err(e);
            }
        }

        let mut all_conditions: Vec<Condition> = Vec::new();
        let conditions_list = block_str.split("conditions=").collect::<Vec<&str>>();
        if conditions_list.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 17")));
        }

        let final_conditions_list = conditions_list[1].trim().split(">").collect::<Vec<&str>>();

        for c_str in final_conditions_list {
            if c_str.len() <= 1 {
                continue;
            }

            if let Err(e) = Condition::from_string(&mut all_conditions, c_str) {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 18: {}", e.1),
                ));
            };
        }

        if let Err(e) = UpdateBlock::create(
            all_blocks,
            global_index,
            block_index,
            ref_col,
            ref_property,
            save,
            add,
            set,
            filter,
        ) {
            return Err(e);
        };

        if let Err(e) = UpdateBlock::set_targets(all_blocks, global_index, all_targets) {
            return Err(e);
        };

        return UpdateBlock::set_conditions(all_blocks, global_index, all_conditions);
    }

    pub fn to_string(block: UpdateBlock) -> String {
        let targets_str = UpdateTarget::stringify(&block.targets);
        let conditions_str = Condition::stringify(&block.conditions);

        let add_str = match block.add {
            Some(a) => RefData::to_string(a),
            None => "".to_string(),
        };

        let set_str = match block.set {
            Some(s) => RefData::to_string(s),
            None => "".to_string(),
        };

        let filter_str = match block.filter {
            Some(f) => Filter::to_string(f),
            None => "".to_string(),
        };

        format!(
            "UPDATE ({},{}) [{},{},{}] {} (add={}) (set={}) {{filter={}}} conditions={}",
            block.global_index,
            block.block_index,
            block.ref_col,
            block.ref_property,
            if block.save == true { "true" } else { "false" },
            targets_str,
            add_str,
            set_str,
            filter_str,
            conditions_str
        )
    }
}
