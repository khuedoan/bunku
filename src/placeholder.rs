use crate::metadata::Metadata;
use crate::resource::ResourceOutputs;
use std::collections::HashMap;

mod substitute;

#[derive(Debug)]
pub struct Context {
    pub metadata: Metadata,
    pub resources: HashMap<String, ResourceOutputs>,
}

pub enum ContextError {}

impl Context {}
