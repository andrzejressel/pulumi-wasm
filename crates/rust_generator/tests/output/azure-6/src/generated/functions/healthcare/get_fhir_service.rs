pub mod get_fhir_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFhirServiceArgs {
        /// The name of the Healthcare FHIR Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The map of tags assigned to the Healthcare FHIR Service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the Healthcare Workspace in which the Healthcare FHIR Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFhirServiceResult {
        /// The list of the access policies of the service instance.
        pub access_policy_object_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The `authentication` block as defined below.
        pub authentications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceAuthentication>,
        >,
        /// The name of the storage account which the operation configuration information is exported to.
        pub configuration_export_storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The list of azure container registry settings used for convert data operation of the service instance.
        pub container_registry_login_server_urls: pulumi_wasm_rust::Output<Vec<String>>,
        /// The `cors` block as defined below.
        pub cors: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceCor>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceIdentity>,
        >,
        /// The kind of the Healthcare FHIR Service.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Healthcare FHIR Service is located.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The map of tags assigned to the Healthcare FHIR Service.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFhirServiceArgs,
    ) -> GetFhirServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let workspace_id_binding = args.workspace_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getFhirService:getFhirService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFhirServiceResult {
            access_policy_object_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessPolicyObjectIds"),
            ),
            authentications: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authentications"),
            ),
            configuration_export_storage_account_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationExportStorageAccountName"),
            ),
            container_registry_login_server_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("containerRegistryLoginServerUrls"),
            ),
            cors: pulumi_wasm_rust::__private::into_domain(o.extract_field("cors")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}
