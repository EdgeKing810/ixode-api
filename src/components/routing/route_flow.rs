use rocket::serde::{Deserialize, Serialize};

use super::blocks::{
    assignment_block::AssignmentBlock, condition_block::ConditionBlock, create_block::CreateBlock,
    fetch_block::FetchBlock, filter_block::FilterBlock, function_block::FunctionBlock,
    loop_block::LoopBlock, object_block::ObjectBlock, property_block::PropertyBlock,
    return_block::ReturnBlock, template_block::TemplateBlock, update_block::UpdateBlock,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteFlow {
    pub fetchers: Vec<FetchBlock>,
    pub assignments: Vec<AssignmentBlock>,
    pub templates: Vec<TemplateBlock>,
    pub conditions: Vec<ConditionBlock>,
    pub loops: Vec<LoopBlock>,
    pub filters: Vec<FilterBlock>,
    pub properties: Vec<PropertyBlock>,
    pub functions: Vec<FunctionBlock>,
    pub objects: Vec<ObjectBlock>,
    pub updates: Vec<UpdateBlock>,
    pub creates: Vec<CreateBlock>,
    pub returns: Vec<ReturnBlock>,
}

impl RouteFlow {
    pub fn create(
        fetch_blocks: Vec<FetchBlock>,
        assignment_blocks: Vec<AssignmentBlock>,
        template_blocks: Vec<TemplateBlock>,
        condition_blocks: Vec<ConditionBlock>,
        loop_blocks: Vec<LoopBlock>,
        filter_blocks: Vec<FilterBlock>,
        property_blocks: Vec<PropertyBlock>,
        function_blocks: Vec<FunctionBlock>,
        object_blocks: Vec<ObjectBlock>,
        update_blocks: Vec<UpdateBlock>,
        create_blocks: Vec<CreateBlock>,
        return_blocks: Vec<ReturnBlock>,
    ) -> RouteFlow {
        RouteFlow {
            fetchers: fetch_blocks,
            assignments: assignment_blocks,
            templates: template_blocks,
            conditions: condition_blocks,
            loops: loop_blocks,
            filters: filter_blocks,
            properties: property_blocks,
            functions: function_blocks,
            objects: object_blocks,
            updates: update_blocks,
            creates: create_blocks,
            returns: return_blocks,
        }
    }

    pub fn add_fetch_block(route_flow: &mut RouteFlow, new_block: FetchBlock) {
        route_flow.fetchers.push(new_block);
    }

    pub fn remove_fetch_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<FetchBlock>::new();
        if block_index >= route_flow.fetchers.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of fetchers present"),
            ));
        }

        for n in 0..route_flow.fetchers.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.fetchers[n].clone());
            }
        }

        route_flow.fetchers = updated_blocks;

        Ok(())
    }

    pub fn set_fetch_blocks(route_flow: &mut RouteFlow, blocks: Vec<FetchBlock>) {
        route_flow.fetchers = blocks;
    }

    pub fn add_assignment_block(route_flow: &mut RouteFlow, new_block: AssignmentBlock) {
        route_flow.assignments.push(new_block);
    }

    pub fn remove_assignment_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<AssignmentBlock>::new();
        if block_index >= route_flow.assignments.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of assignments present"),
            ));
        }

        for n in 0..route_flow.assignments.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.assignments[n].clone());
            }
        }

        route_flow.assignments = updated_blocks;

        Ok(())
    }

    pub fn set_assignment_blocks(route_flow: &mut RouteFlow, blocks: Vec<AssignmentBlock>) {
        route_flow.assignments = blocks;
    }

    pub fn add_template_block(route_flow: &mut RouteFlow, new_block: TemplateBlock) {
        route_flow.templates.push(new_block);
    }

    pub fn remove_template_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<TemplateBlock>::new();
        if block_index >= route_flow.templates.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of templates present"),
            ));
        }

        for n in 0..route_flow.templates.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.templates[n].clone());
            }
        }

        route_flow.templates = updated_blocks;

        Ok(())
    }

    pub fn set_template_blocks(route_flow: &mut RouteFlow, blocks: Vec<TemplateBlock>) {
        route_flow.templates = blocks;
    }

    pub fn add_condition_block(route_flow: &mut RouteFlow, new_block: ConditionBlock) {
        route_flow.conditions.push(new_block);
    }

    pub fn remove_condition_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<ConditionBlock>::new();
        if block_index >= route_flow.conditions.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of conditions present"),
            ));
        }

        for n in 0..route_flow.conditions.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.conditions[n].clone());
            }
        }

        route_flow.conditions = updated_blocks;

        Ok(())
    }

    pub fn set_condition_blocks(route_flow: &mut RouteFlow, blocks: Vec<ConditionBlock>) {
        route_flow.conditions = blocks;
    }

    pub fn add_loop_block(route_flow: &mut RouteFlow, new_block: LoopBlock) {
        route_flow.loops.push(new_block);
    }

    pub fn remove_loop_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<LoopBlock>::new();
        if block_index >= route_flow.loops.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of loops present"),
            ));
        }

        for n in 0..route_flow.loops.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.loops[n].clone());
            }
        }

        route_flow.loops = updated_blocks;

        Ok(())
    }

    pub fn set_loop_blocks(route_flow: &mut RouteFlow, blocks: Vec<LoopBlock>) {
        route_flow.loops = blocks;
    }

    pub fn add_filter_block(route_flow: &mut RouteFlow, new_block: FilterBlock) {
        route_flow.filters.push(new_block);
    }

    pub fn remove_filter_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<FilterBlock>::new();
        if block_index >= route_flow.filters.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of filters present"),
            ));
        }

        for n in 0..route_flow.filters.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.filters[n].clone());
            }
        }

        route_flow.filters = updated_blocks;

        Ok(())
    }

    pub fn set_filter_blocks(route_flow: &mut RouteFlow, blocks: Vec<FilterBlock>) {
        route_flow.filters = blocks;
    }

    pub fn add_property_block(route_flow: &mut RouteFlow, new_block: PropertyBlock) {
        route_flow.properties.push(new_block);
    }

    pub fn remove_property_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<PropertyBlock>::new();
        if block_index >= route_flow.properties.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of properties present"),
            ));
        }

        for n in 0..route_flow.properties.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.properties[n].clone());
            }
        }

        route_flow.properties = updated_blocks;

        Ok(())
    }

    pub fn set_property_blocks(route_flow: &mut RouteFlow, blocks: Vec<PropertyBlock>) {
        route_flow.properties = blocks;
    }

    pub fn add_function_block(route_flow: &mut RouteFlow, new_block: FunctionBlock) {
        route_flow.functions.push(new_block);
    }

    pub fn remove_function_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<FunctionBlock>::new();
        if block_index >= route_flow.functions.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of functions present"),
            ));
        }

        for n in 0..route_flow.functions.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.functions[n].clone());
            }
        }

        route_flow.functions = updated_blocks;

        Ok(())
    }

    pub fn set_function_blocks(route_flow: &mut RouteFlow, blocks: Vec<FunctionBlock>) {
        route_flow.functions = blocks;
    }

    pub fn add_object_block(route_flow: &mut RouteFlow, new_block: ObjectBlock) {
        route_flow.objects.push(new_block);
    }

    pub fn remove_object_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<ObjectBlock>::new();
        if block_index >= route_flow.objects.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of objects present"),
            ));
        }

        for n in 0..route_flow.objects.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.objects[n].clone());
            }
        }

        route_flow.objects = updated_blocks;

        Ok(())
    }

    pub fn set_object_blocks(route_flow: &mut RouteFlow, blocks: Vec<ObjectBlock>) {
        route_flow.objects = blocks;
    }

    pub fn add_update_block(route_flow: &mut RouteFlow, new_block: UpdateBlock) {
        route_flow.updates.push(new_block);
    }

    pub fn remove_update_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut updated_blocks = Vec::<UpdateBlock>::new();
        if block_index >= route_flow.updates.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of updates present"),
            ));
        }

        for n in 0..route_flow.updates.len() {
            if n as u32 != block_index {
                updated_blocks.push(route_flow.updates[n].clone());
            }
        }

        route_flow.updates = updated_blocks;

        Ok(())
    }

    pub fn set_update_blocks(route_flow: &mut RouteFlow, blocks: Vec<UpdateBlock>) {
        route_flow.updates = blocks;
    }

    pub fn add_create_block(route_flow: &mut RouteFlow, new_block: CreateBlock) {
        route_flow.creates.push(new_block);
    }

    pub fn remove_create_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut created_blocks = Vec::<CreateBlock>::new();
        if block_index >= route_flow.creates.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of creates present"),
            ));
        }

        for n in 0..route_flow.creates.len() {
            if n as u32 != block_index {
                created_blocks.push(route_flow.creates[n].clone());
            }
        }

        route_flow.creates = created_blocks;

        Ok(())
    }

    pub fn set_create_blocks(route_flow: &mut RouteFlow, blocks: Vec<CreateBlock>) {
        route_flow.creates = blocks;
    }

    pub fn add_return_block(route_flow: &mut RouteFlow, new_block: ReturnBlock) {
        route_flow.returns.push(new_block);
    }

    pub fn remove_return_block(
        route_flow: &mut RouteFlow,
        block_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut returnd_blocks = Vec::<ReturnBlock>::new();
        if block_index >= route_flow.returns.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of returns present"),
            ));
        }

        for n in 0..route_flow.returns.len() {
            if n as u32 != block_index {
                returnd_blocks.push(route_flow.returns[n].clone());
            }
        }

        route_flow.returns = returnd_blocks;

        Ok(())
    }

    pub fn set_return_blocks(route_flow: &mut RouteFlow, blocks: Vec<ReturnBlock>) {
        route_flow.returns = blocks;
    }

    pub fn from_string(blocks_str: &str) -> Result<RouteFlow, (usize, String)> {
        let current_flow = blocks_str.split("\n").collect::<Vec<&str>>();

        let mut fetch_blocks = Vec::<FetchBlock>::new();
        let mut assignment_blocks = Vec::<AssignmentBlock>::new();
        let mut template_blocks = Vec::<TemplateBlock>::new();
        let mut condition_blocks = Vec::<ConditionBlock>::new();
        let mut loop_blocks = Vec::<LoopBlock>::new();
        let mut filter_blocks = Vec::<FilterBlock>::new();
        let mut property_blocks = Vec::<PropertyBlock>::new();
        let mut function_blocks = Vec::<FunctionBlock>::new();
        let mut object_blocks = Vec::<ObjectBlock>::new();
        let mut update_blocks = Vec::<UpdateBlock>::new();
        let mut create_blocks = Vec::<CreateBlock>::new();
        let mut return_blocks = Vec::<ReturnBlock>::new();

        for line in current_flow {
            if line.trim().len() <= 0 {
                continue;
            }

            if line.starts_with("FETCH") {
                if let Err(e) = FetchBlock::from_string(&mut fetch_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 1: {}", e.1),
                    ));
                }
            } else if line.starts_with("ASSIGN") {
                if let Err(e) = AssignmentBlock::from_string(&mut assignment_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 2: {}", e.1),
                    ));
                }
            } else if line.starts_with("TEMPLATE") {
                if let Err(e) = TemplateBlock::from_string(&mut template_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 3: {}", e.1),
                    ));
                }
            } else if line.starts_with("CONDITION") {
                if let Err(e) = ConditionBlock::from_string(&mut condition_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 4: {}", e.1),
                    ));
                }
            } else if line.starts_with("LOOP") {
                if let Err(e) = LoopBlock::from_string(&mut loop_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 5: {}", e.1),
                    ));
                }
            } else if line.starts_with("FILTER") {
                if let Err(e) = FilterBlock::from_string(&mut filter_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 6: {}", e.1),
                    ));
                }
            } else if line.starts_with("PROPERTY") {
                if let Err(e) = PropertyBlock::from_string(&mut property_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 7: {}", e.1),
                    ));
                }
            } else if line.starts_with("FUNCTION") {
                if let Err(e) = FunctionBlock::from_string(&mut function_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 8: {}", e.1),
                    ));
                }
            } else if line.starts_with("OBJECT") {
                if let Err(e) = ObjectBlock::from_string(&mut object_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 9: {}", e.1),
                    ));
                }
            } else if line.starts_with("UPDATE") {
                if let Err(e) = UpdateBlock::from_string(&mut update_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 10: {}", e.1),
                    ));
                }
            } else if line.starts_with("CREATE") {
                if let Err(e) = CreateBlock::from_string(&mut create_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 11: {}", e.1),
                    ));
                }
            } else if line.starts_with("RETURN") {
                if let Err(e) = ReturnBlock::from_string(&mut return_blocks, line) {
                    return Err((
                        500,
                        format!("Error: Invalid blocks_str string / 12: {}", e.1),
                    ));
                }
            }
        }

        Ok(RouteFlow::create(
            fetch_blocks,
            assignment_blocks,
            template_blocks,
            condition_blocks,
            loop_blocks,
            filter_blocks,
            property_blocks,
            function_blocks,
            object_blocks,
            update_blocks,
            create_blocks,
            return_blocks,
        ))
    }

    pub fn to_string(route_flow: RouteFlow) -> String {
        let mut blocks_str = String::new();

        blocks_str = format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            blocks_str,
            FetchBlock::stringify(&route_flow.fetchers),
            AssignmentBlock::stringify(&route_flow.assignments),
            TemplateBlock::stringify(&route_flow.templates),
            ConditionBlock::stringify(&route_flow.conditions),
            LoopBlock::stringify(&route_flow.loops),
            FilterBlock::stringify(&route_flow.filters),
            PropertyBlock::stringify(&route_flow.properties),
            FunctionBlock::stringify(&route_flow.functions),
            ObjectBlock::stringify(&route_flow.objects),
            UpdateBlock::stringify(&route_flow.updates),
            CreateBlock::stringify(&route_flow.creates),
            ReturnBlock::stringify(&route_flow.returns),
        );

        blocks_str
    }
}
