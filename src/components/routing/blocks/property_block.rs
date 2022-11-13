use rocket::serde::{Deserialize, Serialize};

use crate::{components::{routing::submodules::{
    sub_property::Property, sub_property_apply::PropertyApply, sub_ref_data::RefData,
}, constraint_property::ConstraintProperty}, utils::{mapping::auto_fetch_all_mappings, constraint::auto_fetch_all_constraints}};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub property: Property,
}

impl PropertyBlock {
    pub fn create(
        all_blocks: &mut Vec<PropertyBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
        data: RefData,
        apply: &str,
        additional: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = PropertyBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            property: Property::default(),
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
            let property_update =
                Self::update_property(all_blocks, global_index, data, apply, additional);
            if let Err(e) = property_update {
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

    pub fn exist(all_blocks: &Vec<PropertyBlock>, global_index: u32) -> bool {
        let mut found = false;
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_blocks: &Vec<PropertyBlock>, global_index: u32) -> Option<PropertyBlock> {
        for block in all_blocks.iter() {
            if block.global_index == global_index {
                return Some(block.clone());
            }
        }

        None
    }

    pub fn update_local_name(
        all_blocks: &mut Vec<PropertyBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<PropertyBlock> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
            let final_value = match ConstraintProperty::validate(&all_constraints, "property_block", "local_name", local_name) {
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
            return Err((404, String::from("Error: Property Block not found")));
        }

        Ok(())
    }

    pub fn update_property(
        all_blocks: &mut Vec<PropertyBlock>,
        global_index: u32,
        data: RefData,
        apply: &str,
        additional: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<PropertyBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                match Property::create(data, apply, additional) {
                    Ok(p) => block.property = p,
                    Err(e) => return Err(e),
                };

                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Property Block not found")));
        }

        Ok(())
    }

    pub fn set_property(
        all_blocks: &mut Vec<PropertyBlock>,
        global_index: u32,
        property: Property,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<PropertyBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.property = property;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Property Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<PropertyBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<PropertyBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Property Block not found")));
        }

        let updated_blocks: Vec<PropertyBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| PropertyBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                property: block.property.clone(),
            })
            .collect::<Vec<PropertyBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<PropertyBlock>) -> String {
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
                PropertyBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<PropertyBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("PROPERTY (").collect::<Vec<&str>>();
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

        let property_tmp_str = block_str.split("]").collect::<Vec<&str>>();
        if property_tmp_str.len() <= 1 {
            return Err((500, String::from("at start of property declaration")));
        }

        let property_str = property_tmp_str[1..].join("]");

        let property = match Property::from_string(&property_str) {
            Ok(p) => p,
            Err(e) => return Err((500, format!("while processing property -> {}", e.1))),
        };

        match PropertyBlock::create(
            all_blocks,
            global_index,
            block_index,
            local_name,
            property.data,
            &PropertyApply::to(property.apply),
            &property.additional,
        ) {
            Ok(_) => Ok(()),
            Err(e) => return Err((500, format!("while processing block -> {}", e.1))),
        }
    }

    pub fn to_string(block: PropertyBlock) -> String {
        format!(
            "PROPERTY ({},{}) [{}] {}",
            block.global_index,
            block.block_index,
            block.local_name,
            Property::to_string(block.property),
        )
    }
}
