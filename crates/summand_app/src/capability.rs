use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SummandCapabilityType {
    Add,
    Remove,
}

impl fmt::Display for SummandCapabilityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SummandCapabilityType::Add => "AddCapability",
                SummandCapabilityType::Remove => "RemoveCapability",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct SummandCapability {
    pub name: String,
    pub capability_type: SummandCapabilityType,
    pub version: String,
}

impl SummandCapability {
    pub fn new(name: String, capability_type: SummandCapabilityType, version: String) -> Self {
        Self {
            name: name,
            capability_type: capability_type,
            version: version,
        }
    }
}

impl fmt::Display for SummandCapability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Summand Capability(name: {}, version: {:?}, type: {:?})",
            self.name, self.version, self.capability_type
        )
    }
}
