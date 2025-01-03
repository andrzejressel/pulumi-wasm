/// Manages an AWS Opensearch Package.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   myOpensearchPackages:
///     type: aws:s3:BucketV2
///     name: my_opensearch_packages
///     properties:
///       bucket: my-opensearch-packages
///   example:
///     type: aws:s3:BucketObjectv2
///     properties:
///       bucket: ${myOpensearchPackages.bucket}
///       key: example.txt
///       source:
///         fn::FileAsset: ./example.txt
///       etag:
///         fn::invoke:
///           function: std:filemd5
///           arguments:
///             input: ./example.txt
///           return: result
///   examplePackage:
///     type: aws:opensearch:Package
///     name: example
///     properties:
///       packageName: example-txt
///       packageSource:
///         s3BucketName: ${myOpensearchPackages.bucket}
///         s3Key: ${example.key}
///       packageType: TXT-DICTIONARY
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS Opensearch Packages using the Package ID. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/package:Package example package-id
/// ```
pub mod package {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PackageArgs {
        /// Description of the package.
        #[builder(into, default)]
        pub package_description: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique name for the package.
        #[builder(into)]
        pub package_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the package source options.
        #[builder(into)]
        pub package_source: pulumi_wasm_rust::Output<
            super::super::types::opensearch::PackagePackageSource,
        >,
        /// The type of package.
        #[builder(into)]
        pub package_type: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PackageResult {
        /// The current version of the package.
        pub available_package_version: pulumi_wasm_rust::Output<String>,
        /// Description of the package.
        pub package_description: pulumi_wasm_rust::Output<Option<String>>,
        pub package_id: pulumi_wasm_rust::Output<String>,
        /// Unique name for the package.
        pub package_name: pulumi_wasm_rust::Output<String>,
        /// Configuration block for the package source options.
        pub package_source: pulumi_wasm_rust::Output<
            super::super::types::opensearch::PackagePackageSource,
        >,
        /// The type of package.
        pub package_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PackageArgs) -> PackageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let package_description_binding = args.package_description.get_inner();
        let package_name_binding = args.package_name.get_inner();
        let package_source_binding = args.package_source.get_inner();
        let package_type_binding = args.package_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:opensearch/package:Package".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "packageDescription".into(),
                    value: &package_description_binding,
                },
                register_interface::ObjectField {
                    name: "packageName".into(),
                    value: &package_name_binding,
                },
                register_interface::ObjectField {
                    name: "packageSource".into(),
                    value: &package_source_binding,
                },
                register_interface::ObjectField {
                    name: "packageType".into(),
                    value: &package_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "availablePackageVersion".into(),
                },
                register_interface::ResultField {
                    name: "packageDescription".into(),
                },
                register_interface::ResultField {
                    name: "packageId".into(),
                },
                register_interface::ResultField {
                    name: "packageName".into(),
                },
                register_interface::ResultField {
                    name: "packageSource".into(),
                },
                register_interface::ResultField {
                    name: "packageType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PackageResult {
            available_package_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availablePackageVersion").unwrap(),
            ),
            package_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageDescription").unwrap(),
            ),
            package_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageId").unwrap(),
            ),
            package_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageName").unwrap(),
            ),
            package_source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageSource").unwrap(),
            ),
            package_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageType").unwrap(),
            ),
        }
    }
}
