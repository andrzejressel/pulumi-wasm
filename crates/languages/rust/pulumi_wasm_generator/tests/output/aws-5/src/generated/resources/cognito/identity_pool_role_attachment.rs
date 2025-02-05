/// Provides an AWS Cognito Identity Pool Roles Attachment.
///
/// ## Import
///
/// Using `pulumi import`, import Cognito Identity Pool Roles Attachment using the Identity Pool ID. For example:
///
/// ```sh
/// $ pulumi import aws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment example us-west-2:b64805ad-cb56-40ba-9ffc-f5d8207e6d42
/// ```
pub mod identity_pool_role_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolRoleAttachmentArgs {
        /// An identity pool ID in the format `REGION_GUID`.
        #[builder(into)]
        pub identity_pool_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A List of Role Mapping.
        #[builder(into, default)]
        pub role_mappings: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMapping>,
            >,
        >,
        /// The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
        #[builder(into)]
        pub roles: pulumi_wasm_rust::InputOrOutput<
            std::collections::HashMap<String, String>,
        >,
    }
    #[allow(dead_code)]
    pub struct IdentityPoolRoleAttachmentResult {
        /// An identity pool ID in the format `REGION_GUID`.
        pub identity_pool_id: pulumi_wasm_rust::Output<String>,
        /// A List of Role Mapping.
        pub role_mappings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMapping>,
            >,
        >,
        /// The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
        pub roles: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IdentityPoolRoleAttachmentArgs,
    ) -> IdentityPoolRoleAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_pool_id_binding = args
            .identity_pool_id
            .get_output(context)
            .get_inner();
        let role_mappings_binding = args.role_mappings.get_output(context).get_inner();
        let roles_binding = args.roles.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cognito/identityPoolRoleAttachment:IdentityPoolRoleAttachment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityPoolId".into(),
                    value: &identity_pool_id_binding,
                },
                register_interface::ObjectField {
                    name: "roleMappings".into(),
                    value: &role_mappings_binding,
                },
                register_interface::ObjectField {
                    name: "roles".into(),
                    value: &roles_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IdentityPoolRoleAttachmentResult {
            identity_pool_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityPoolId"),
            ),
            role_mappings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleMappings"),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(o.extract_field("roles")),
        }
    }
}
