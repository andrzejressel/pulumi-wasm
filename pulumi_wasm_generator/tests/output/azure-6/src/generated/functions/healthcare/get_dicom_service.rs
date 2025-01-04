pub mod get_dicom_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDicomServiceArgs {
        /// The name of the Healthcare DICOM Service
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The id of the Healthcare Workspace in which the Healthcare DICOM Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDicomServiceResult {
        /// The `authentication` block as defined below.
        pub authentications: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServiceAuthentication>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServiceIdentity>,
        >,
        /// The Azure Region where the Healthcare DICOM Service is located.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServicePrivateEndpoint>,
        >,
        /// The url of the Healthcare DICOM Services.
        pub service_url: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the Healthcare DICOM Service.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDicomServiceArgs) -> GetDicomServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getDicomService:getDicomService".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authentications".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "privateEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "serviceUrl".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDicomServiceResult {
            authentications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authentications").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            private_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateEndpoints").unwrap(),
            ),
            service_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceUrl").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
