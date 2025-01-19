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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IdentityPoolRoleAttachmentArgs {
        /// An identity pool ID in the format `REGION_GUID`.
        #[builder(into)]
        pub identity_pool_id: pulumi_wasm_rust::Output<String>,
        /// A List of Role Mapping.
        #[builder(into, default)]
        pub role_mappings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::cognito::IdentityPoolRoleAttachmentRoleMapping>,
            >,
        >,
        /// The map of roles associated with this pool. For a given role, the key will be either "authenticated" or "unauthenticated" and the value will be the Role ARN.
        #[builder(into)]
        pub roles: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
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
        name: &str,
        args: IdentityPoolRoleAttachmentArgs,
    ) -> IdentityPoolRoleAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_pool_id_binding = args.identity_pool_id.get_inner();
        let role_mappings_binding = args.role_mappings.get_inner();
        let roles_binding = args.roles.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "identityPoolId".into(),
                },
                register_interface::ResultField {
                    name: "roleMappings".into(),
                },
                register_interface::ResultField {
                    name: "roles".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IdentityPoolRoleAttachmentResult {
            identity_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityPoolId").unwrap(),
            ),
            role_mappings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleMappings").unwrap(),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roles").unwrap(),
            ),
        }
    }
}
