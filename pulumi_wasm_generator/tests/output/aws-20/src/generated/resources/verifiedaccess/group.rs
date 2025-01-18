/// Resource for managing a Verified Access Group.
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
///     let example = group::create(
///         "example",
///         GroupArgs::builder()
///             .verifiedaccess_instance_id("${exampleAwsVerifiedaccessInstance.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Usage with KMS Key
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = group::create(
///         "test",
///         GroupArgs::builder()
///             .sse_configuration(
///                 GroupSseConfiguration::builder()
///                     .kmsKeyArn("${testKey.arn}")
///                     .build_struct(),
///             )
///             .verifiedaccess_instance_id(
///                 "${testAwsVerifiedaccessInstanceTrustProviderAttachment.verifiedaccessInstanceId}",
///             )
///             .build_struct(),
///     );
///     let testKey = key::create(
///         "testKey",
///         KeyArgs::builder()
///             .description("KMS key for Verified Access Group test")
///             .build_struct(),
///     );
/// }
/// ```
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// Description of the verified access group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy document that is associated with this resource.
        #[builder(into, default)]
        pub policy_document: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use KMS keys for server-side encryption.
        #[builder(into, default)]
        pub sse_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::verifiedaccess::GroupSseConfiguration>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the verified access instance this group is associated with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub verifiedaccess_instance_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// Timestamp when the access group was created.
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Timestamp when the access group was deleted.
        pub deletion_time: pulumi_wasm_rust::Output<String>,
        /// Description of the verified access group.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Timestamp when the access group was last updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// AWS account number owning this resource.
        pub owner: pulumi_wasm_rust::Output<String>,
        /// The policy document that is associated with this resource.
        pub policy_document: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block to use KMS keys for server-side encryption.
        pub sse_configuration: pulumi_wasm_rust::Output<
            super::super::types::verifiedaccess::GroupSseConfiguration,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of this verified acess group.
        pub verifiedaccess_group_arn: pulumi_wasm_rust::Output<String>,
        /// ID of this verified access group.
        pub verifiedaccess_group_id: pulumi_wasm_rust::Output<String>,
        /// The id of the verified access instance this group is associated with.
        ///
        /// The following arguments are optional:
        pub verifiedaccess_instance_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GroupArgs) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let policy_document_binding = args.policy_document.get_inner();
        let sse_configuration_binding = args.sse_configuration.get_inner();
        let tags_binding = args.tags.get_inner();
        let verifiedaccess_instance_id_binding = args
            .verifiedaccess_instance_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:verifiedaccess/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "policyDocument".into(),
                    value: &policy_document_binding,
                },
                register_interface::ObjectField {
                    name: "sseConfiguration".into(),
                    value: &sse_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "verifiedaccessInstanceId".into(),
                    value: &verifiedaccess_instance_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "deletionTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "policyDocument".into(),
                },
                register_interface::ResultField {
                    name: "sseConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "verifiedaccessGroupArn".into(),
                },
                register_interface::ResultField {
                    name: "verifiedaccessGroupId".into(),
                },
                register_interface::ResultField {
                    name: "verifiedaccessInstanceId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupResult {
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            deletion_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            policy_document: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDocument").unwrap(),
            ),
            sse_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sseConfiguration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            verifiedaccess_group_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedaccessGroupArn").unwrap(),
            ),
            verifiedaccess_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedaccessGroupId").unwrap(),
            ),
            verifiedaccess_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verifiedaccessInstanceId").unwrap(),
            ),
        }
    }
}
