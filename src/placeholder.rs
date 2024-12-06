use crate::metadata::Metadata;
use crate::resource::{Resource, ResourceOutputs};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub metadata: Metadata,
    pub resources: HashMap<String, Resource>,
}

pub enum ContextError {}

impl Context {
    pub fn new(
        metadata: Metadata,
        _resources: HashMap<String, ResourceOutputs>,
    ) -> Result<Context, ContextError> {
        Ok(Context {
            metadata,
            resources: HashMap::new(),
        })
    }
}
