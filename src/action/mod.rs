pub mod base;
pub mod meta;

use serde::{Deserialize, Serialize};

pub trait ActionError: std::error::Error + Send + Sync {
    fn boxed(self) -> Box<dyn std::error::Error + Send + Sync>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<E> ActionError for E where E: std::error::Error + Send + Sized + Sync {}

#[async_trait::async_trait]
#[typetag::serde(tag = "action")]
pub trait Action: Send + Sync + std::fmt::Debug + dyn_clone::DynClone {
    fn describe_execute(&self) -> Vec<ActionDescription>;
    fn describe_revert(&self) -> Vec<ActionDescription>;

    // They should also have an `async fn plan(args...) -> Result<ActionState<Self>, Self::Error>;`
    async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn revert(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

dyn_clone::clone_trait_object!(Action);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ActionState {
    Completed,
    // Only applicable to meta-actions that start multiple sub-actions.
    Progress,
    Uncompleted,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]

pub struct ActionDescription {
    pub description: String,
    pub explanation: Vec<String>,
}

impl ActionDescription {
    fn new(description: String, explanation: Vec<String>) -> Self {
        Self {
            description,
            explanation,
        }
    }
}