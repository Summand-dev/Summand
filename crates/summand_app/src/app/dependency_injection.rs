use std::any::{type_name_of_val, Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub trait Injectable: Downcast + Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
impl_downcast!(Injectable);

impl<T: Any + Send + Sync> Injectable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

// impl Debug for dyn Injectable {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Try to print if the inner type is Debug
//         if let Some(debuggable) = self.downcast_ref::<&(dyn Debug + 'static)>() {
//             write!(f, "{:?}", debuggable)
//         } else if let Some(debuggable) = self.as_any().downcast_ref::<&(dyn Debug)>() {
//             write!(f, "{:?}", debuggable)
//         } else {
//             write!(f, "Injectable type: {:?}", type_name_of_val(self))
//         }
//     }
// }

#[derive(Debug)]
pub enum InjectionType {
    Singletone,
    New,
}

pub struct InjectionBinding {
    pub injection_type: InjectionType,
    pub initiator: Box<dyn Fn() -> Box<dyn Injectable>>,
}

pub struct DependencyInjector {
    pub bindings: HashMap<String, InjectionBinding>,
    pub instances: HashMap<String, Box<dyn Injectable>>,
}

impl DependencyInjector {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            instances: HashMap::new(),
        }
    }

    pub fn bind<T: Injectable + 'static>(
        &mut self,
        key: &str,
        injection_type: InjectionType,
        initiator: fn() -> T,
    ) {
        self.bindings.insert(
            key.to_string(),
            InjectionBinding {
                injection_type,
                initiator: Box::new(move || Box::new(initiator()) as Box<dyn Injectable>),
            },
        );
    }

    fn create_instance<T: Injectable + 'static>(&mut self, key: &str, binding: &InjectionBinding) {
        let instance = (binding.initiator)();
        self.instances.insert(key.to_string(), instance);
    }

    pub fn resolve<T: Injectable + Debug + 'static>(&mut self, key: &str) -> Option<&T> {
        match self.bindings.get(key) {
            Some(binding) => match binding.injection_type {
                InjectionType::Singletone => {
                    if !self.instances.contains_key(key) {
                        let instance = (binding.initiator)();
                        // println!("Created new instance {:?}", instance);
                        self.instances.insert(key.to_string(), instance);
                    }
                    let original = self
                        .instances
                        .get(key)
                        .and_then(|boxed| boxed.downcast_ref::<T>());
                    println!("Origian type {:?}", original);
                    original
                }
                InjectionType::New => {
                    let instance = (binding.initiator)();
                    self.instances.insert(key.to_string(), instance);
                    self.instances
                        .get(key)
                        .and_then(|boxed| boxed.downcast_ref::<T>())
                }
            },
            None => None,
        }
    }

    /// Safer version of `resolve` that returns an owned instance
    pub fn resolve_owned<T: Injectable + 'static>(&self, key: &str) -> Option<Box<T>> {
        self.bindings.get(key).map(|binding| {
            let boxed = (binding.initiator)();
            boxed.downcast::<T>().ok()
        })?
    }
}

impl Default for DependencyInjector {
    fn default() -> Self {
        Self::new()
    }
}
