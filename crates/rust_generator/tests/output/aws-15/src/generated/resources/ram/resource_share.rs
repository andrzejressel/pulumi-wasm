/// Manages a Resource Access Manager (RAM) Resource Share. To associate principals with the share, see the `aws.ram.PrincipalAssociation` resource. To associate resources with the share, see the `aws.ram.ResourceAssociation` resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ram:ResourceShare
///     properties:
///       name: example
///       allowExternalPrincipals: true
///       tags:
///         Environment: Production
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import resource shares using the `arn` of the resource share. For example:
///
/// ```sh
/// $ pulumi import aws:ram/resourceShare:ResourceShare example arn:aws:ram:eu-west-1:123456789012:resource-share/73da1ab9-b94a-4ba3-8eb4-45917f7f4b12
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resource_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceShareArgs {
        /// Indicates whether principals outside your organization can be associated with a resource share.
        #[builder(into, default)]
        pub allow_external_principals: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource share.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
        #[builder(into, default)]
        pub permission_arns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResourceShareResult {
        /// Indicates whether principals outside your organization can be associated with a resource share.
        pub allow_external_principals: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Resource Name (ARN) of the resource share.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource share.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Amazon Resource Names (ARNs) of the RAM permission to associate with the resource share. If you do not specify an ARN for the permission, RAM automatically attaches the default version of the permission for each resource type. You can associate only one permission with each resource type included in the resource share.
        pub permission_arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A map of tags to assign to the resource share. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ResourceShareArgs,
    ) -> ResourceShareResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_external_principals_binding = args
            .allow_external_principals
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let permission_arns_binding = args.permission_arns.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ram/resourceShare:ResourceShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowExternalPrincipals".into(),
                    value: allow_external_principals_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "permissionArns".into(),
                    value: permission_arns_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResourceShareResult {
            allow_external_principals: o.get_field("allowExternalPrincipals"),
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            permission_arns: o.get_field("permissionArns"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
