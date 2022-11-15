use model::service::*;
use model::types::*;

#[path = "user/endpoints.rs"]
mod user_endpoints;

#[path = "user/pg_func.rs"]
mod user_pg_func;


pub fn get_services() -> Vec<Service> {
    vec![
        Service::new("user", 2, user_endpoints::get_user_endpoints()),
    ]
}

pub fn get_proc_functions() -> Vec<ProceduralFunction> {
    vec![
        user_pg_func::get_user_pg_func(),
    ]
    .concat()
}
