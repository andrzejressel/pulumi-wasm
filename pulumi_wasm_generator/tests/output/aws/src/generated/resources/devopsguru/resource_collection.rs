/// Resource for managing an AWS DevOps Guru Resource Collection.
///
/// > Only one type of resource collection (All Account Resources, CloudFormation, or Tags) can be enabled in an account at a time. To avoid persistent differences, this resource should be defined only once.
///
/// ## Example Usage
///
/// ### All Account Resources
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .cloudformation(
///                 ResourceCollectionCloudformation::builder()
///                     .stackNames(vec!["*",])
///                     .build_struct(),
///             )
///             .type_("AWS_SERVICE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### CloudFormation Stacks
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .cloudformation(
///                 ResourceCollectionCloudformation::builder()
///                     .stackNames(vec!["ExampleStack",])
///                     .build_struct(),
///             )
///             .type_("AWS_CLOUD_FORMATION")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tags
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .tags(
///                 ResourceCollectionTags::builder()
///                     .appBoundaryKey("DevOps-Guru-Example")
///                     .tagValues(vec!["Example-Value",])
///                     .build_struct(),
///             )
///             .type_("AWS_TAGS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Tags All Resources
///
/// To analyze all resources with the `app_boundary_key` regardless of the corresponding tag value, set `tag_values` to `["*"]`.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_collection::create(
///         "example",
///         ResourceCollectionArgs::builder()
///             .tags(
///                 ResourceCollectionTags::builder()
///                     .appBoundaryKey("DevOps-Guru-Example")
///                     .tagValues(vec!["*",])
///                     .build_struct(),
///             )
///             .type_("AWS_TAGS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DevOps Guru Resource Collection using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:devopsguru/resourceCollection:ResourceCollection example AWS_CLOUD_FORMATION
/// ```
pub mod resource_collection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceCollectionArgs {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        #[builder(into, default)]
        pub cloudformation: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionCloudformation>,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionTags>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ResourceCollectionResult {
        /// A collection of AWS CloudFormation stacks. See `cloudformation` below for additional details.
        pub cloudformation: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionCloudformation>,
        >,
        /// AWS tags used to filter the resources in the resource collection. See `tags` below for additional details.
        pub tags: pulumi_wasm_rust::Output<
            Option<super::super::types::devopsguru::ResourceCollectionTags>,
        >,
        /// Type of AWS resource collection to create. Valid values are `AWS_CLOUD_FORMATION`, `AWS_SERVICE`, and `AWS_TAGS`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResourceCollectionArgs) -> ResourceCollectionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloudformation_binding = args.cloudformation.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devopsguru/resourceCollection:ResourceCollection".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudformation".into(),
                    value: &cloudformation_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudformation".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceCollectionResult {
            cloudformation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudformation").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
