use crate::{CompatibilityOperator, PackageName, VersionSpecification, PackageMetadata, MetaRequirement};

impl PackageName {
    pub fn new(name: String) -> Self {
        PackageName { name }
    }
}

impl VersionSpecification {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        // ! pre and build neglected
        VersionSpecification { major, minor, patch, pre: None, build: None }
    }
}

impl Default for VersionSpecification {
    fn default() -> Self {
        VersionSpecification {
            major: 0,
            minor: 0,
            patch: 0,
            pre: None,
            build: None,
        }
    }
}

impl PartialEq for VersionSpecification {
    fn eq(&self, other: &Self) -> bool {
        // ! No implementation for pre and build
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for VersionSpecification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // ! pre and build neglected
        if self.major != other.major {
            Some(self.major.cmp(&other.major))
        } else if self.minor != other.minor {
            Some(self.minor.cmp(&other.minor))
        } else if self.patch != other.patch {
            Some(self.patch.cmp(&other.patch))
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl PackageMetadata {
    pub fn new(meta_requirement: MetaRequirement, compatibility: CompatibilityOperator, version: VersionSpecification) -> Self {
        PackageMetadata { meta_requirement, compatibility, version }
    }
}

impl Default for PackageMetadata {
    fn default() -> Self {
        PackageMetadata {
            meta_requirement: MetaRequirement::Undefined,
            compatibility: CompatibilityOperator::Undefined,
            version: VersionSpecification::default()
        }
    }
}