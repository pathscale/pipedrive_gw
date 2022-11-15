use model::endpoint::*;
use model::types::{Field, Type};

pub fn endpoint_user_add_crm_lead() -> EndpointSchema {
    EndpointSchema::new(
        "AddCrmLead",
        20660,
        vec![
            Field::new("email", Type::String),
            Field::new("username", Type::String),
            Field::new("title", Type::String),
            Field::new("message", Type::String),
        ],
        vec![],
    )
}

pub fn get_user_endpoints() -> Vec<EndpointSchema> {
    vec![
        endpoint_user_add_crm_lead(),
    ]
}
