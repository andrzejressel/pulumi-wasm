/// Provides an AWS Cognito Identity Pool Roles Attachment.
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment example us-west-2:b64805ad-cb56-40ba-9ffc-f5d8207e6d42
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod identity_pool_role_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolRoleAttachmentArgs {
        /// An identity pool ID in the format `REGION_GUID`.
        #[builder(into)]
        pub identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A List of Role Mapping.
        #[builder(into, default)]
        pub role_mappings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMapping>,
            >,
        >,
        /// The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
        #[builder(into)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolRoleAttachmentResult {
        /// An identity pool ID in the format `REGION_GUID`.
        pub identity_pool_id: pulumi_gestalt_rust::Output<String>,
        /// A List of Role Mapping.
        pub role_mappings: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMapping>,
            >,
        >,
        /// The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
        pub roles: pulumi_gestalt_rust::Output<
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
        args: IdentityPoolRoleAttachmentArgs,
    ) -> IdentityPoolRoleAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_pool_id_binding = args.identity_pool_id.get_output(context);
        let role_mappings_binding = args.role_mappings.get_output(context);
        let roles_binding = args.roles.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityPoolId".into(),
                    value: &identity_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleMappings".into(),
                    value: &role_mappings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roles".into(),
                    value: &roles_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IdentityPoolRoleAttachmentResult {
            identity_pool_id: o.get_field("identityPoolId"),
            role_mappings: o.get_field("roleMappings"),
            roles: o.get_field("roles"),
        }
    }
}
