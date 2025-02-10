/// Provides an Amazon Connect Security Profile resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:SecurityProfile
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: example
///       description: example description
///       permissions:
///         - BasicAgentAccess
///         - OutboundCallAccess
///       tags:
///         Name: Example Security Profile
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Security Profiles using the `instance_id` and `security_profile_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/securityProfile:SecurityProfile example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod security_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SecurityProfileArgs {
        /// Specifies the description of the Security Profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Security Profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of permissions assigned to the security profile.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Tags to apply to the Security Profile. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SecurityProfileResult {
        /// The Amazon Resource Name (ARN) of the Security Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the description of the Security Profile.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Security Profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The organization resource identifier for the security profile.
        pub organization_resource_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of permissions assigned to the security profile.
        pub permissions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The identifier for the Security Profile.
        pub security_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Security Profile. If configured with a provider
        /// `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SecurityProfileArgs,
    ) -> SecurityProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let permissions_binding = args.permissions.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:connect/securityProfile:SecurityProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissions".into(),
                    value: permissions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SecurityProfileResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            instance_id: o.get_field("instanceId"),
            name: o.get_field("name"),
            organization_resource_id: o.get_field("organizationResourceId"),
            permissions: o.get_field("permissions"),
            security_profile_id: o.get_field("securityProfileId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
