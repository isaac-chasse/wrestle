use crate::{CompatibilityOperator, PackageName, VersionSpecification, PackageMetadata, MetaRequirement, PackageRequirement};
use core::fmt::{self, Display};

impl Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for VersionSpecification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Display for CompatibilityOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let compatibility_str = match self {
            CompatibilityOperator::Exact => "==",
            CompatibilityOperator::Greater => ">",
            CompatibilityOperator::GreaterEq => ">=",
            CompatibilityOperator::Less => "<",
            CompatibilityOperator::LessEq => "<=",
            CompatibilityOperator::Caret => "^",
            CompatibilityOperator::Tilde => "~=",
            CompatibilityOperator::Wildcard => "*",
            CompatibilityOperator::Undefined => todo!(),
        };

        write!(f, "{}", compatibility_str)
    }
}

impl Display for MetaRequirement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta_requirement_str = match self {
            MetaRequirement::LanguageVersion => "python_version",
            MetaRequirement::PlatformVersion => "platform_system",
            MetaRequirement::Undefined => todo!()
        };

        write!(f, "{}", meta_requirement_str)
    }
}

impl Display for PackageMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // * Noteably, Metadata has spaces
        write!(f, "{} {} {}", self.meta_requirement, self.compatibility, self.version)
    }
}

impl Display for PackageRequirement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.metadata {
            Some(metadata) => write!(f, "{}{}{} ;  {}", self.name, self.compatibility, self.version, metadata),
            None => write!(f, "{}{}{}", self.name, self.compatibility, self.version)
        }
    }
}