use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::sub_filter::Filter;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub ref_var: String,
    pub ref_property: String,
    pub filters: Vec<Filter>,
}

impl FilterBlock {
    pub fn create(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
        ref_var: &str,
        ref_property: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = FilterBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            ref_var: "".to_string(),
            ref_property: "".to_string(),
            filters: vec![],
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
            let ref_var_update = Self::update_ref_var(all_blocks, global_index, ref_var);
            if let Err(e) = ref_var_update {
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

        if has_error {
            let delete_block = Self::delete(all_blocks, global_index);
            if let Err(e) = delete_block {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_blocks: &Vec<FilterBlock>, global_index: u32) -> bool {
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
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

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
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn update_ref_var(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        ref_var: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

        if !String::from(ref_var)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: ref_var contains an invalid character"),
            ));
        }

        if String::from(ref_var.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: ref_var does not contain enough characters"),
            ));
        } else if String::from(ref_var.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: ref_var contains too many characters"),
            ));
        }

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.ref_var = ref_var.trim().to_string();
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn update_ref_property(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        ref_property: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

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
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn add_filter(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        new_filter: Filter,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.filters.push(new_filter);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn remove_filter(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        filter_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_filters = Vec::<Filter>::new();
                if filter_index >= block.filters.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of filters present"),
                    ));
                }

                for n in 0..block.filters.len() {
                    if n as u32 != filter_index {
                        updated_filters.push(block.filters[n].clone());
                    }
                }

                block.filters = updated_filters;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn set_filters(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
        filters: Vec<Filter>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.filters = filters;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Filter Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<FilterBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<FilterBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Filter Block not found")));
        }

        let updated_blocks: Vec<FilterBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| FilterBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                ref_var: block.ref_var.clone(),
                ref_property: block.ref_property.clone(),
                filters: block.filters.clone(),
            })
            .collect::<Vec<FilterBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<FilterBlock>) -> String {
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
                FilterBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<FilterBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("FILTER (").collect::<Vec<&str>>();
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

        let local_name = current_block[0];
        let ref_var = current_block[1];
        let ref_property = current_block[2];

        let mut all_filters: Vec<Filter> = Vec::new();
        let filters_list = block_str.split("]").collect::<Vec<&str>>();
        if filters_list.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 9")));
        }

        let filters_list_str = filters_list[1..].join("]");
        let final_filters_list = filters_list_str.trim().split(">").collect::<Vec<&str>>();

        for f_str in final_filters_list {
            if let Err(e) = Filter::from_string(&mut all_filters, f_str) {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 10: {}", e.1),
                ));
            };
        }

        if let Err(e) = FilterBlock::create(
            all_blocks,
            global_index,
            block_index,
            local_name,
            ref_var,
            ref_property,
        ) {
            return Err(e);
        };

        return FilterBlock::set_filters(all_blocks, global_index, all_filters);
    }

    pub fn to_string(block: FilterBlock) -> String {
        let filters_str = Filter::stringify(&block.filters);

        format!(
            "FILTER ({},{}) [{},{},{}] {}",
            block.global_index,
            block.block_index,
            block.local_name,
            block.ref_var,
            block.ref_property,
            filters_str
        )
    }
}
