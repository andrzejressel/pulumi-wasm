#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod credential_user_managed_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityArgs {
        /// (Optional) List of string annotations.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        #[builder(into)]
        pub data_factory_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// (Optional) Short text description
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        #[builder(into)]
        pub identity_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The desired name of the credential resource
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityResult {
        /// (Optional) List of string annotations.
        pub annotations: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        pub data_factory_id: pulumi_gestalt_rust::Output<String>,
        /// (Optional) Short text description
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        pub identity_id: pulumi_gestalt_rust::Output<String>,
        /// The desired name of the credential resource
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CredentialUserManagedIdentityArgs,
    ) -> CredentialUserManagedIdentityResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let data_factory_id_binding = args.data_factory_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let identity_id_binding = args.identity_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:datafactory/credentialUserManagedIdentity:CredentialUserManagedIdentity"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataFactoryId".into(),
                    value: data_factory_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityId".into(),
                    value: identity_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CredentialUserManagedIdentityResult {
            annotations: o.get_field("annotations"),
            data_factory_id: o.get_field("dataFactoryId"),
            description: o.get_field("description"),
            identity_id: o.get_field("identityId"),
            name: o.get_field("name"),
        }
    }
}
