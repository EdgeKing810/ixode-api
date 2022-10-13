use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::{sub_condition::Condition, sub_ref_data::RefData};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateBlock {
    pub global_index: u32,
    pub block_index: u32,
    pub local_name: String,
    pub template: String,
    pub data: Vec<RefData>,
    pub conditions: Vec<Condition>,
}

impl TemplateBlock {
    pub fn create(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        block_index: u32,
        local_name: &str,
        template: &str,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_block = TemplateBlock {
            global_index: global_index,
            block_index: block_index,
            local_name: "".to_string(),
            template: "".to_string(),
            data: vec![],
            conditions: vec![],
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
            let template_update = Self::update_template(all_blocks, global_index, template);
            if let Err(e) = template_update {
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

    pub fn exist(all_blocks: &Vec<TemplateBlock>, global_index: u32) -> bool {
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
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        local_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

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
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn update_template(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        template: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        if String::from(template.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: template does not contain enough characters"),
            ));
        } else if String::from(template.trim()).len() > 1000 {
            return Err((
                400,
                String::from("Error: template contains too many characters"),
            ));
        }

        let mut new_template = template.split("\n").collect::<Vec<&str>>().join(" ");
        new_template = new_template
            .split("template=")
            .collect::<Vec<&str>>()
            .join(" ");

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.template = new_template.trim().to_string();
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn add_data(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        new_data: RefData,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.data.push(new_data);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn remove_data(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        data_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());

                let mut updated_data = Vec::<RefData>::new();
                if data_index >= block.data.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of data present"),
                    ));
                }

                for n in 0..block.data.len() {
                    if n as u32 != data_index {
                        updated_data.push(block.data[n].clone());
                    }
                }

                block.data = updated_data;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn set_data(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        data: Vec<RefData>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.data = data;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        new_condition: Condition,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions.push(new_condition);
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

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
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
        conditions: Vec<Condition>,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                block.conditions = conditions;
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_blocks: &mut Vec<TemplateBlock>,
        global_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_block: Option<TemplateBlock> = None;

        for block in all_blocks.iter_mut() {
            if block.global_index == global_index {
                found_block = Some(block.clone());
                break;
            }
        }

        if let None = found_block {
            return Err((404, String::from("Error: Template Block not found")));
        }

        let updated_blocks: Vec<TemplateBlock> = all_blocks
            .iter_mut()
            .filter(|block| block.global_index != global_index)
            .map(|block| TemplateBlock {
                global_index: block.global_index,
                block_index: block.block_index,
                local_name: block.local_name.clone(),
                template: block.template.clone(),
                data: block.data.clone(),
                conditions: block.conditions.clone(),
            })
            .collect::<Vec<TemplateBlock>>();

        *all_blocks = updated_blocks;

        Ok(())
    }

    pub fn stringify(all_blocks: &Vec<TemplateBlock>) -> String {
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
                TemplateBlock::to_string(block.clone()),
            );
        }

        stringified_blocks
    }

    pub fn from_string(
        all_blocks: &mut Vec<TemplateBlock>,
        block_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_block = block_str.split("TEMPLATE (").collect::<Vec<&str>>();
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

        let local_name = current_block[0];

        let mut all_conditions: Vec<Condition> = Vec::new();

        current_block = block_str.split("{conditions=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 8")));
        }

        current_block = current_block[1].split("}").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 9")));
        }

        let conditions_list = current_block[0].trim().split(">").collect::<Vec<&str>>();

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

        let mut all_data: Vec<RefData> = Vec::new();

        current_block = block_str.split("(data=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 11")));
        }

        current_block = current_block[1].split(")").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 12")));
        }

        let data_list = current_block[0].trim().split(">").collect::<Vec<&str>>();

        for d_str in data_list {
            if d_str.len() < 1 {
                continue;
            }

            match RefData::from_string(d_str) {
                Ok(rfd) => all_data.push(rfd),
                Err(e) => {
                    return Err((
                        500,
                        format!("Error: Invalid block_str string / 13: {}", e.1),
                    ))
                }
            }
        }

        current_block = block_str.split("template=").collect::<Vec<&str>>();
        if current_block.len() <= 1 {
            return Err((500, String::from("Error: Invalid block_str string / 14")));
        }

        let template = current_block[1].trim();

        match TemplateBlock::create(all_blocks, global_index, block_index, local_name, template) {
            Ok(f) => f,
            Err(e) => {
                return Err((
                    500,
                    format!("Error: Invalid block_str string / 15: {}", e.1),
                ))
            }
        };

        if let Err(e) = TemplateBlock::set_data(all_blocks, global_index, all_data) {
            return Err((
                500,
                format!("Error: Invalid block_str string / 16: {}", e.1),
            ));
        }

        match TemplateBlock::set_conditions(all_blocks, global_index, all_conditions) {
            Ok(_) => Ok(()),
            Err(e) => Err((
                500,
                format!("Error: Invalid block_str string / 17: {}", e.1),
            )),
        }
    }

    pub fn to_string(block: TemplateBlock) -> String {
        let conditions_str = Condition::stringify(&block.conditions);

        let mut stringified_data = String::new();

        for data in block.data {
            stringified_data = format!(
                "{}{}{}",
                stringified_data,
                if stringified_data.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                RefData::to_string(data.clone()),
            );
        }

        format!(
            "TEMPLATE ({},{}) [{}] (data={}) {{conditions={}}} template={}",
            block.global_index,
            block.block_index,
            block.local_name,
            stringified_data,
            conditions_str,
            block.template
        )
    }
}