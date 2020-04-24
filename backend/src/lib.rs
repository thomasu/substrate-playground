#![feature(async_closure, proc_macro_hygiene, decl_macro)]
#![deny(intra_doc_link_resolution_failure)]

pub mod api;
pub mod kubernetes;
pub mod manager;
pub mod metrics;

use crate::manager::Manager;

pub struct Context {
    pub manager: Manager,
}