// DONE Define basic PackageRequirements struct
// DONE - Built from three basics 1) PackageName 2) CombatibilityOperator 3) VersionSpecification
// DONE - Include an Optional PackageMetadata which covers lang, sys, and platform version
// DONE Define PackageName
// DONE - Simple string storage for name
// DONE - Build an ordering operation
// DONE Define CompatibilityOperators
// DONE - Match statements for extracting strings
// DONE Define VersionSpecification
// DONE - Major, Minor, Micro for Python
// DONE - Need to fix this to handle an enum because we are getting issues already
// DONE - SimpleVersion, SemanticVersion, Undefined
// DONE - Actually I should probably just move to SemanticVersion with Optional Pre and Build specs
// TODO Error handling
mod impls;
mod display;

// PackageRequirement
#[derive(Debug, Clone)]
pub struct PackageRequirement {
    pub name: PackageName,
    pub compatibility: CompatibilityOperator,
    pub version: VersionSpecification,
    pub metadata: Option<PackageMetadata>,
}

// PackageName for storing the package names
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PackageName {
    pub name: String
}

// CompatibilityOperator for the comparisons
#[derive(Clone, Debug, PartialEq)]
pub enum CompatibilityOperator {
    Exact,
    GreaterEq,
    LessEq,
    Greater,
    Less,
    Tilde,
    Caret,
    Wildcard,
    Undefined,
}

// VersionSpecification for checking versions
// Picks a Semantic Style for verbosity
#[derive(Clone, Debug)]
pub struct VersionSpecification {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<Prerelease>,
    pub build: Option<BuildMetadata>,
}

// Hold empty prelease and build metadata struct for now
#[derive(Clone, Debug)]
pub struct Prerelease;

#[derive(Clone, Debug)]
pub struct BuildMetadata;


// PackageMetadata
// ! This should also build to a Vec<MetaData> since there is often more than one
// ! Versioning is incomplete and does not cover the gamut of information possible
// ! MetaData versions might include other version semantics such as semver
#[derive(Clone, Debug, PartialEq)]
pub struct PackageMetadata {
    pub meta_requirement: MetaRequirement,
    pub compatibility: CompatibilityOperator,
    pub version: VersionSpecification,
}

// MetaRequirement
// ! Non-exhaustive and needs to be handled 
#[derive(Clone, Debug, PartialEq)]
pub enum MetaRequirement {
    LanguageVersion,
    PlatformVersion,
    Undefined,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_simple_package() {
        let pkg_name = PackageName::new("silly".to_string());
        let pkg_comp = CompatibilityOperator::Exact;
        let pkg_spec = VersionSpecification::new(1, 0, 4);
        
        assert_eq!(pkg_name.name, String::from("silly"));
        assert_eq!(pkg_comp, CompatibilityOperator::Exact);
        assert_eq!(pkg_spec, VersionSpecification::new(1, 0, 4));
    }

    #[test]
    fn test_check_names() {
        let expected_name = PackageName::new("silly".to_string());
        assert_eq!(expected_name, PackageName{ name: String::from("silly") });
        assert_ne!(expected_name, PackageName{ name: String::from("not_silly") })
    }

    #[test]
    fn test_check_ops() {
        let exact_op = CompatibilityOperator::Exact;
        let greater_op = CompatibilityOperator::Greater;
        let greatereq_op = CompatibilityOperator::GreaterEq;
        let less_op = CompatibilityOperator::Less;
        let lesseq_op = CompatibilityOperator::LessEq;
        let caret_op = CompatibilityOperator::Caret;
        let tilde_op = CompatibilityOperator::Tilde;
        let wildcard_op = CompatibilityOperator::Wildcard;
        let undefined_op = CompatibilityOperator::Undefined;
    
        // Ensure the operators have the correct values
        assert_eq!(exact_op, CompatibilityOperator::Exact);
        assert_eq!(greater_op, CompatibilityOperator::Greater);
        assert_eq!(greatereq_op, CompatibilityOperator::GreaterEq);
        assert_eq!(less_op, CompatibilityOperator::Less);
        assert_eq!(lesseq_op, CompatibilityOperator::LessEq);
        assert_eq!(caret_op, CompatibilityOperator::Caret);
        assert_eq!(tilde_op, CompatibilityOperator::Tilde);
        assert_eq!(wildcard_op, CompatibilityOperator::Wildcard);
        assert_eq!(undefined_op, CompatibilityOperator::Undefined);
    
        // Ensure the operators are distinct from each other
        assert_ne!(exact_op, greater_op);
        assert_ne!(exact_op, greatereq_op);
        assert_ne!(exact_op, less_op);
        assert_ne!(exact_op, lesseq_op);
        assert_ne!(exact_op, caret_op);
        assert_ne!(exact_op, tilde_op);
        assert_ne!(exact_op, wildcard_op);
        assert_ne!(exact_op, undefined_op);
    }

    #[test]
    fn test_check_versions() {
        let version_spec_a = VersionSpecification::new(1, 1, 12);
        let version_spec_b = VersionSpecification::new(1, 2, 12);
        let version_spec_c = VersionSpecification::new(2, 2, 12);
        assert_eq!(version_spec_a, VersionSpecification::new(1, 1, 12));
        assert_eq!(version_spec_b, VersionSpecification::new(1, 2, 12));
        assert_eq!(version_spec_c, VersionSpecification::new(2, 2, 12));
        assert_ne!(version_spec_b, version_spec_c);
    }

    #[test]
    fn test_package_name_ordering() {
        let package1 = PackageName::new("abc".to_string());
        let package2 = PackageName::new("def".to_string());

        assert!(package1 < package2);
        assert!(package1 <= package2);
        assert!(package2 > package1);
        assert!(package2 >= package1);
    }

    #[test]
    fn test_version_spec_ordering() {
        let version_spec_a = VersionSpecification::new(1, 1, 12);
        let version_spec_b = VersionSpecification::new(1, 2, 12);
        let version_spec_c = VersionSpecification::new(2, 2, 12);

        assert!(version_spec_a < version_spec_b);  // 1.1.12 < 1.2.12
        assert!(version_spec_a < version_spec_c);  // 1.1.12 < 2.2.12
        assert!(version_spec_b < version_spec_c);  // 1.2.12 < 2.2.12

        assert!(version_spec_b > version_spec_a);  // 1.2.12 > 1.1.12
        assert!(version_spec_c > version_spec_a);  // 2.2.12 > 1.1.12
        assert!(version_spec_c > version_spec_b);  // 2.2.12 > 1.2.12

        assert!(version_spec_a <= version_spec_b); // 1.1.12 <= 1.2.12
        assert!(version_spec_a <= version_spec_c); // 1.1.12 <= 2.2.12
        assert!(version_spec_b <= version_spec_c); // 1.2.12 <= 2.2.12

        assert!(version_spec_b >= version_spec_a); // 1.2.12 >= 1.1.12
        assert!(version_spec_c >= version_spec_a); // 2.2.12 >= 1.1.12
        assert!(version_spec_c >= version_spec_b); // 2.2.12 >= 1.2.12

        assert_eq!(version_spec_a, version_spec_a); // 1.1.12 == 1.1.12
        assert_eq!(version_spec_b, version_spec_b); // 1.2.12 == 1.2.12
        assert_eq!(version_spec_c, version_spec_c); // 2.2.12 == 2.2.12
    }

    #[test]
    fn test_package_metadata() {
        let package_requirement_without_metadata = PackageRequirement {
            name: PackageName::new("package1".to_string()),
            compatibility: CompatibilityOperator::Exact,
            version: VersionSpecification::new(1, 0, 0),
            metadata: None,
        };

        let package_requirement_with_metadata = PackageRequirement {
            name: PackageName::new("package2".to_string()),
            compatibility: CompatibilityOperator::Wildcard,
            version: VersionSpecification::new(2, 0, 0),
            metadata: Some(PackageMetadata {
                meta_requirement: MetaRequirement::LanguageVersion,
                compatibility: CompatibilityOperator::Greater,
                version: VersionSpecification::new(2, 1, 0),
            }),
        };

        // Test package requirement without metadata
        assert_eq!(package_requirement_without_metadata.metadata, None);

        // Test package requirement with metadata
        assert_eq!(
            package_requirement_with_metadata.metadata,
            Some(PackageMetadata {
                meta_requirement: MetaRequirement::LanguageVersion,
                compatibility: CompatibilityOperator::Greater,
                version: VersionSpecification::new(2, 1, 0),
            })
        );
    }


}
