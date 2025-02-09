#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_account_encryption {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAccountEncryptionArgs {
        /// The ID of the NetApp account where customer managed keys-based encryption is enabled.
        #[builder(into)]
        pub netapp_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAccountEncryptionResult {
        pub encryption_key: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub netapp_account_id: pulumi_gestalt_rust::Output<String>,
        pub system_assigned_identity_principal_id: pulumi_gestalt_rust::Output<String>,
        pub user_assigned_identity_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAccountEncryptionArgs,
    ) -> GetAccountEncryptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let netapp_account_id_binding = args.netapp_account_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getAccountEncryption:getAccountEncryption".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "netappAccountId".into(),
                    value: netapp_account_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAccountEncryptionResult {
            encryption_key: o.get_field("encryptionKey"),
            id: o.get_field("id"),
            netapp_account_id: o.get_field("netappAccountId"),
            system_assigned_identity_principal_id: o
                .get_field("systemAssignedIdentityPrincipalId"),
            user_assigned_identity_id: o.get_field("userAssignedIdentityId"),
        }
    }
}
