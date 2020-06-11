#![feature(async_closure, proc_macro_hygiene, decl_macro)]
#![deny(intra_doc_link_resolution_failure)]

mod api;
mod controller;
mod kubernetes;
mod manager;
mod metrics;
mod puller;
mod template;

pub trait System {
} 