use std::{collections::BTreeMap, error::Error, thread};

use crate::{
    objects::{BehindHandle, Object, ObjectSet},
    storage::ObjectId,
    validate::ValidationError,
};

use super::State;

/// Errors that occurred while validating the objects inserted into the stores
#[derive(Default)]
pub struct Validation {
    /// All unhandled validation errors
    pub errors: BTreeMap<ObjectId, ValidationError>,
}

impl Drop for Validation {
    fn drop(&mut self) {
        let num_errors = self.errors.len();
        if num_errors > 0 {
            println!(
                "Dropping `Validation` with {num_errors} unhandled validation \
                errors:"
            );

            for err in self.errors.values() {
                println!("{}", err);

                // Once `Report` is stable, we can replace this:
                // https://doc.rust-lang.org/std/error/struct.Report.html
                let mut source = err.source();
                while let Some(err) = source {
                    println!("Caused by:\n\t{err}");
                    source = err.source();
                }
            }

            if !thread::panicking() {
                panic!();
            }
        }
    }
}

impl State for Validation {
    type Command = ValidationCommand;
    type Event = ValidationEvent;

    fn decide(&self, command: Self::Command, events: &mut Vec<Self::Event>) {
        let mut errors = Vec::new();

        match command {
            ValidationCommand::ValidateObject { object } => {
                object.validate(&mut errors);

                for err in errors {
                    events.push(ValidationEvent::ValidationFailed {
                        object: object.clone(),
                        err,
                    });
                }
            }
            ValidationCommand::OnlyValidate { objects } => {
                events.push(ValidationEvent::ClearErrors);

                for object in objects {
                    object.validate(&mut errors);

                    for err in errors.drain(..) {
                        events.push(ValidationEvent::ValidationFailed {
                            object: object.clone(),
                            err,
                        });
                    }
                }
            }
        }
    }

    fn evolve(&mut self, event: &Self::Event) {
        match event {
            ValidationEvent::ValidationFailed { object, err } => {
                self.errors.insert(object.id(), err.clone());
            }
            ValidationEvent::ClearErrors => self.errors.clear(),
        }
    }
}

/// The command accepted by the validation service
pub enum ValidationCommand {
    /// Validate the provided object
    ValidateObject {
        /// The object to validate
        object: Object<BehindHandle>,
    },

    /// Validate the provided objects, discard all other validation errors
    OnlyValidate {
        /// The objects to validate
        objects: ObjectSet,
    },
}

/// The event produced by the validation service
#[derive(Clone)]
pub enum ValidationEvent {
    /// Validation of an object failed
    ValidationFailed {
        /// The object for which validation failed
        object: Object<BehindHandle>,

        /// The validation error
        err: ValidationError,
    },

    /// All stored validation errors are being cleared
    ClearErrors,
}
