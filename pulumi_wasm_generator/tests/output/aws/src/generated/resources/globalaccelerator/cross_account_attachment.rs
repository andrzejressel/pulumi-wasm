/// Resource for managing an AWS Global Accelerator Cross Account Attachment.
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
///     let example = cross_account_attachment::create(
///         "example",
///         CrossAccountAttachmentArgs::builder()
///             .name("example-cross-account-attachment")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with Optional Arguments
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cross_account_attachment::create(
///         "example",
///         CrossAccountAttachmentArgs::builder()
///             .name("example-cross-account-attachment")
///             .principals(vec!["123456789012",])
///             .resources(
///                 vec![
///                     CrossAccountAttachmentResource::builder()
///                     .endpointId("arn:aws:elasticloadbalancing:us-west-2:123456789012:loadbalancer/app/my-load-balancer/50dc6c495c0c9188")
///                     .region("us-west-2").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator Cross Account Attachment using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/crossAccountAttachment:CrossAccountAttachment example arn:aws:globalaccelerator::012345678910:attachment/01234567-abcd-8910-efgh-123456789012
/// ```
pub mod cross_account_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CrossAccountAttachmentArgs {
        /// Name of the Cross Account Attachment.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// List of AWS account IDs that are allowed to associate resources with the accelerator.
        #[builder(into, default)]
        pub principals: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of resources to be associated with the accelerator.
        #[builder(into, default)]
        pub resources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CrossAccountAttachmentResource,
                >,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CrossAccountAttachmentResult {
        /// ARN of the Cross Account Attachment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Creation Time when the Cross Account Attachment.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// Last modified time of the Cross Account Attachment.
        pub last_modified_time: pulumi_wasm_rust::Output<String>,
        /// Name of the Cross Account Attachment.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of AWS account IDs that are allowed to associate resources with the accelerator.
        pub principals: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of resources to be associated with the accelerator.
        pub resources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::globalaccelerator::CrossAccountAttachmentResource,
                >,
            >,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CrossAccountAttachmentArgs,
    ) -> CrossAccountAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let principals_binding = args.principals.get_inner();
        let resources_binding = args.resources.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:globalaccelerator/crossAccountAttachment:CrossAccountAttachment"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "principals".into(),
                    value: &principals_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "principals".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CrossAccountAttachmentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principals").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}