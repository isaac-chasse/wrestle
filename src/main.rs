use vergen::{CompatibilityOperator, MetaRequirement, PackageMetadata, PackageName, PackageRequirement, VersionSpecification};

fn main() {
    // Sample data
    let package_name = PackageName { name: "example-package".to_owned() };
    let compatibility_operator = CompatibilityOperator::Exact;
    let version_specification = VersionSpecification::new(1, 2, 3);
    let metadata = Some(PackageMetadata {
        meta_requirement: MetaRequirement::LanguageVersion,
        compatibility: CompatibilityOperator::GreaterEq,
        version: VersionSpecification::new(3, 2, 1),
    });

    let package_requirement = PackageRequirement {
        name: package_name,
        compatibility: compatibility_operator,
        version: version_specification,
        metadata: metadata,
    };

    // Printing structs
    // println!("Package Name: {}", package_name);
    // println!("Compatibility Operator: {}", compatibility_operator);
    // println!("Version Specification: {}", version_specification);
    // println!("Package Metadata: {}", metadata.unwrap_or_default());
    println!("Package Requirement: {}", package_requirement);
}