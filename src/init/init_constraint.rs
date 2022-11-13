use crate::{
    components::{
        constraint::{fetch_all_constraints, save_all_constraints, Constraint},
        constraint_property::ConstraintProperty,
        mappings::{get_file_name, Mapping},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_constraints(mappings: &Vec<Mapping>) -> Vec<Constraint> {
    let all_constraints_path = get_file_name("constraints", mappings);
    let mut all_constraints = Vec::<Constraint>::new();

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    if let Err(e) = all_constraints_path {
        println!("{}", e);
        return all_constraints;
    }

    all_constraints = match fetch_all_constraints(
        all_constraints_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e.1);
            return all_constraints;
        }
    };

    if !Constraint::exist(&all_constraints, "collection") {
        if let Err(e) = Constraint::create(&mut all_constraints, "collection") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "project_id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "description",
            false,
            false,
            1,
            400,
            vec![';', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "collection", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "config") {
        if let Err(e) = Constraint::create(&mut all_constraints, "config") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "value",
            false,
            false,
            1,
            200,
            vec!['|'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "config", all_properties) {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "custom_structure") {
        if let Err(e) = Constraint::create(&mut all_constraints, "custom_structure") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "description",
            false,
            false,
            0,
            1000,
            vec![';', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "custom_structure", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "data") {
        if let Err(e) = Constraint::create(&mut all_constraints, "data") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "project_id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "collection_id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "data", all_properties) {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "datapair") {
        if let Err(e) = Constraint::create(&mut all_constraints, "datapair") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "structure_id",
            true,
            true,
            1,
            200,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "custom_structure_id",
            true,
            true,
            0,
            200,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "value",
            false,
            false,
            0,
            500000,
            vec!['|'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "dtype",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "datapair", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "event") {
        if let Err(e) = Constraint::create(&mut all_constraints, "event") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "event_type",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "description",
            false,
            false,
            1,
            1000,
            vec![';'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "redirect",
            false,
            false,
            1,
            200,
            vec![';'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "event", all_properties) {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "media") {
        if let Err(e) = Constraint::create(&mut all_constraints, "media") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            500,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            false,
            false,
            1,
            500,
            vec!['^'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "media", all_properties) {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "project") {
        if let Err(e) = Constraint::create(&mut all_constraints, "project") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "description",
            false,
            false,
            0,
            400,
            vec![';'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "api_path",
            true,
            true,
            1,
            50,
            vec![],
            vec!['-', '_', '/'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "members",
            true,
            true,
            1,
            50,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "project", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "structure") {
        if let Err(e) = Constraint::create(&mut all_constraints, "structure") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "description",
            false,
            false,
            0,
            1000,
            vec![';', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "stype",
            false,
            false,
            0,
            100,
            vec![';', '@', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "default_val",
            false,
            false,
            0,
            99999999,
            vec![';', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "regex_pattern",
            false,
            false,
            0,
            1000,
            vec![';', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "structure", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "user") {
        if let Err(e) = Constraint::create(&mut all_constraints, "user") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "first_name",
            true,
            false,
            1,
            100,
            vec![],
            vec!['-', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "last_name",
            true,
            false,
            1,
            100,
            vec![],
            vec!['-', ' '],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "username",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "email",
            false,
            false,
            1,
            100,
            vec![';'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "password",
            false,
            false,
            7,
            100,
            vec![';'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "user", all_properties) {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "route_component") {
        if let Err(e) = Constraint::create(&mut all_constraints, "route_component") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "route_id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "route_path",
            true,
            true,
            1,
            200,
            vec![],
            vec!['-', '_', '/'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "project_id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "route_component", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "auth_jwt") {
        if let Err(e) = Constraint::create(&mut all_constraints, "auth_jwt") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "field",
            true,
            true,
            0,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_col",
            true,
            true,
            0,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "auth_jwt", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "body_data") {
        if let Err(e) = Constraint::create(&mut all_constraints, "body_data") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "bdtype",
            false,
            false,
            1,
            100,
            vec![';', '@', '>', '#'],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "body_data", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "param_data") {
        if let Err(e) = Constraint::create(&mut all_constraints, "param_data") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "delimiter",
            true,
            true,
            0,
            5,
            vec![],
            vec!['&', '!', '#', '-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "param_data", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "assignment_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "assignment_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "assignment_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "create_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "create_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_col",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_object",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "create_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "fetch_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "fetch_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_col",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "fetch_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "filter_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "filter_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_var",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_property",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_', '.'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "filter_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "function_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "function_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "function_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "loop_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "loop_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "loop_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "object_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "object_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "object_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "property_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "property_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "property_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "template_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "template_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "local_name",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "template",
            false,
            false,
            1,
            1000,
            vec![],
            vec![],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "template_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "update_block") {
        if let Err(e) = Constraint::create(&mut all_constraints, "update_block") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_col",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "ref_property",
            true,
            true,
            0,
            100,
            vec![],
            vec!['-', '_', '.'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "update_block", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "fail_obj") {
        if let Err(e) = Constraint::create(&mut all_constraints, "fail_obj") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "message",
            true,
            true,
            0,
            200,
            vec![],
            vec!['-', '_', ':', ';', ' ', '.', '/'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "fail_obj", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "object_pair") {
        if let Err(e) = Constraint::create(&mut all_constraints, "object_pair") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "id",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "object_pair", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "property") {
        if let Err(e) = Constraint::create(&mut all_constraints, "property") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "additional",
            true,
            true,
            0,
            100,
            vec![],
            vec!['-', '_', '.'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "property", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "ref_data") {
        if let Err(e) = Constraint::create(&mut all_constraints, "ref_data") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "data",
            true,
            true,
            0,
            200,
            vec![],
            vec!['-', '_', ':', ';', ' ', '.'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) = Constraint::set_properties(&mut all_constraints, "ref_data", all_properties)
        {
            println!("{}", e.1);
        }
    }

    if !Constraint::exist(&all_constraints, "update_target") {
        if let Err(e) = Constraint::create(&mut all_constraints, "update_target") {
            println!("{}", e.1);
        }

        let mut all_properties = Vec::<ConstraintProperty>::new();
        if let Err(e) = ConstraintProperty::create(
            &mut all_properties,
            "field",
            true,
            true,
            1,
            100,
            vec![],
            vec!['-', '_'],
        ) {
            println!("{}", e.1);
        }

        if let Err(e) =
            Constraint::set_properties(&mut all_constraints, "update_target", all_properties)
        {
            println!("{}", e.1);
        }
    }

    save_all_constraints(
        &all_constraints,
        all_constraints_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_constraints
}
