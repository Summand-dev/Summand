use std::collections::HashMap;
use std::fmt::Debug;

use crate::{compatiblity::{SummandCompatiblity, SummandCompatiblityType}, workspace::SummandWorkspace};

#[derive(Debug, Clone)]
pub struct SummandValidationError {
    pub message: String,
}

impl std::fmt::Display for SummandValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SummandValidationError: {}", self.message)
    }
}

impl std::error::Error for SummandValidationError {}

pub trait SummandValidator: SummandValidatorClone + Debug {
    fn validate(&self, workspace: &SummandWorkspace, compatiblity: &SummandCompatiblity) -> Result<(), SummandValidationError>;
}

#[derive(Clone, Debug)]
pub struct SummandPlatformValidator {}

impl SummandValidator for SummandPlatformValidator {
    fn validate(
        &self,
        workspace: &SummandWorkspace,
        compatiblity: &SummandCompatiblity,
    ) -> Result<(), SummandValidationError> {
        let os_name = &workspace.env_variables["OS_NAME"];

        if !compatiblity.allow.is_empty() && !compatiblity.allow.contains(os_name) {
            return Err(SummandValidationError {
                message: format!("OS '{}' is not in allow list", os_name),
            });
        }

        if !compatiblity.disallow.is_empty() && compatiblity.disallow.contains(os_name) {
            return Err(SummandValidationError {
                message: format!("OS '{}' is in disallow list", os_name),
            });
        }

        Ok(())
    }
}

pub trait SummandValidatorClone {
    fn clone_box(&self) -> Box<dyn SummandValidator>;
}

impl<T> SummandValidatorClone for T
where
    T: 'static + SummandValidator + Clone,
{
    fn clone_box(&self) -> Box<dyn SummandValidator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SummandValidator> {
    fn clone(&self) -> Box<dyn SummandValidator> {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct SummandValidatorFactory {
    validators: HashMap<SummandCompatiblityType, Box<dyn SummandValidator>>,
}

impl SummandValidatorFactory {
    pub fn validate_all(
        &self,
        workspace: &SummandWorkspace,
        compatiblities: &[SummandCompatiblity],
    ) -> Result<(), SummandValidationError> {
        for c in compatiblities {
            self.validate(workspace, c)?; // Will early return the first error
        }
        Ok(())
    }

    pub fn validate(
        &self,
        workspace: &SummandWorkspace,
        compatiblity: &SummandCompatiblity,
    ) -> Result<(), SummandValidationError> {
        if let Some(validator) = self.validators.get(&compatiblity.compatiblity_type) {
            validator.validate(workspace, compatiblity)
        } else {
            Ok(()) // No validator registered for this type, assume valid
        }
    }
}
