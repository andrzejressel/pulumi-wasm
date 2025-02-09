#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccessPolicyArgs {
        /// Specifies the name of the Management Template. Possible values are: `Key Management`,
        /// `Secret Management`, `Certificate Management`, `Key & Secret Management`, `Key & Certificate Management`,
        /// `Secret & Certificate Management`,  `Key, Secret, & Certificate Management`
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccessPolicyResult {
        /// the certificate permissions for the access policy
        pub certificate_permissions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// the key permissions for the access policy
        pub key_permissions: pulumi_gestalt_rust::Output<Vec<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// the secret permissions for the access policy
        pub secret_permissions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccessPolicyArgs,
    ) -> GetAccessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getAccessPolicy:getAccessPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccessPolicyResult {
            certificate_permissions: o.get_field("certificatePermissions"),
            id: o.get_field("id"),
            key_permissions: o.get_field("keyPermissions"),
            name: o.get_field("name"),
            secret_permissions: o.get_field("secretPermissions"),
        }
    }
}
