/// Manages an AWS Opensearch Package Association.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = package::create(
///         "example",
///         PackageArgs::builder()
///             .package_name("example-txt")
///             .package_source(
///                 PackagePackageSource::builder()
///                     .s3BucketName("${myOpensearchPackages.bucket}")
///                     .s3Key("${exampleAwsS3Object.key}")
///                     .build_struct(),
///             )
///             .package_type("TXT-DICTIONARY")
///             .build_struct(),
///     );
///     let examplePackageAssociation = package_association::create(
///         "examplePackageAssociation",
///         PackageAssociationArgs::builder()
///             .domain_name("${myDomain.domainName}")
///             .package_id("${example.id}")
///             .build_struct(),
///     );
///     let myDomain = domain::create(
///         "myDomain",
///         DomainArgs::builder()
///             .cluster_config(
///                 DomainClusterConfig::builder()
///                     .instanceType("r4.large.search")
///                     .build_struct(),
///             )
///             .domain_name("my-opensearch-domain")
///             .engine_version("Elasticsearch_7.10")
///             .build_struct(),
///     );
/// }
/// ```
pub mod package_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PackageAssociationArgs {
        /// Name of the domain to associate the package with.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Internal ID of the package to associate with a domain.
        #[builder(into)]
        pub package_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PackageAssociationResult {
        /// Name of the domain to associate the package with.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// Internal ID of the package to associate with a domain.
        pub package_id: pulumi_wasm_rust::Output<String>,
        pub reference_path: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PackageAssociationArgs) -> PackageAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let package_id_binding = args.package_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/packageAssociation:PackageAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "packageId".into(),
                    value: &package_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "packageId".into(),
                },
                register_interface::ResultField {
                    name: "referencePath".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PackageAssociationResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            package_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageId").unwrap(),
            ),
            reference_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("referencePath").unwrap(),
            ),
        }
    }
}
