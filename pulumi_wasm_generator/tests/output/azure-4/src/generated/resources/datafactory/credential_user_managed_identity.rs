pub mod credential_user_managed_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityArgs {
        /// (Optional) List of string annotations.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        #[builder(into)]
        pub data_factory_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// (Optional) Short text description
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        #[builder(into)]
        pub identity_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The desired name of the credential resource
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CredentialUserManagedIdentityResult {
        /// (Optional) List of string annotations.
        pub annotations: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The resource ID of the parent Data Factory
        pub data_factory_id: pulumi_wasm_rust::Output<String>,
        /// (Optional) Short text description
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the User Assigned Managed Identity
        pub identity_id: pulumi_wasm_rust::Output<String>,
        /// The desired name of the credential resource
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CredentialUserManagedIdentityArgs,
    ) -> CredentialUserManagedIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_output(context).get_inner();
        let data_factory_id_binding = args
            .data_factory_id
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let identity_id_binding = args.identity_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datafactory/credentialUserManagedIdentity:CredentialUserManagedIdentity"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "dataFactoryId".into(),
                    value: &data_factory_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CredentialUserManagedIdentityResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("annotations"),
            ),
            data_factory_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dataFactoryId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            identity_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
