/// Resource for managing a Verified Access Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// Description of the verified access group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy document that is associated with this resource.
        #[builder(into, default)]
        pub policy_document: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block to use KMS keys for server-side encryption.
        #[builder(into, default)]
        pub sse_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::verifiedaccess::GroupSseConfiguration>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the verified access instance this group is associated with.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub verifiedaccess_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// Timestamp when the access group was created.
        pub creation_time: pulumi_gestalt_rust::Output<String>,
        /// Timestamp when the access group was deleted.
        pub deletion_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the verified access group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Timestamp when the access group was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// AWS account number owning this resource.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// The policy document that is associated with this resource.
        pub policy_document: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block to use KMS keys for server-side encryption.
        pub sse_configuration: pulumi_gestalt_rust::Output<
            super::super::types::verifiedaccess::GroupSseConfiguration,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of this verified acess group.
        pub verifiedaccess_group_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of this verified access group.
        pub verifiedaccess_group_id: pulumi_gestalt_rust::Output<String>,
        /// The id of the verified access instance this group is associated with.
        ///
        /// The following arguments are optional:
        pub verifiedaccess_instance_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let policy_document_binding_1 = args.policy_document.get_output(context);
        let policy_document_binding = policy_document_binding_1.get_inner();
        let sse_configuration_binding_1 = args.sse_configuration.get_output(context);
        let sse_configuration_binding = sse_configuration_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let verifiedaccess_instance_id_binding_1 = args
            .verifiedaccess_instance_id
            .get_output(context);
        let verifiedaccess_instance_id_binding = verifiedaccess_instance_id_binding_1
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupResult {
            creation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            deletion_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            policy_document: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDocument"),
            ),
            sse_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sseConfiguration"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            verifiedaccess_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("verifiedaccessGroupArn"),
            ),
            verifiedaccess_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("verifiedaccessGroupId"),
            ),
            verifiedaccess_instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("verifiedaccessInstanceId"),
            ),
        }
    }
}
