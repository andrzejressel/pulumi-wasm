/// Manages a Healthcare DICOM Service
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: azure:healthcare:Workspace
///     properties:
///       name: tfexworkspace
///       resourceGroupName: tfex-resource_group
///       location: east us
///   testDicomService:
///     type: azure:healthcare:DicomService
///     name: test
///     properties:
///       name: tfexDicom
///       workspaceId: ${test.id}
///       location: east us
///       identity:
///         type: SystemAssigned
///       tags:
///         environment: None
/// ```
///
/// ## Import
///
/// Healthcare DICOM Service can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/dicomService:DicomService example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1/dicomServices/service1
/// ```
///
pub mod dicom_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DicomServiceArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::DicomServiceIdentity>,
        >,
        /// Specifies the Azure Region where the Healthcare DICOM Service should be created. Changing this forces a new Healthcare DICOM Service to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Healthcare DICOM Service. Changing this forces a new Healthcare DICOM Service to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether to enabled public networks when data plane traffic coming from public networks while private endpoint is enabled. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the Healthcare DICOM Service.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare DICOM Service should exist. Changing this forces a new Healthcare DICOM Service to be created.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DicomServiceResult {
        /// The `authentication` block as defined below.
        pub authentications: pulumi_wasm_rust::Output<
            Vec<super::super::types::healthcare::DicomServiceAuthentication>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::healthcare::DicomServiceIdentity>,
        >,
        /// Specifies the Azure Region where the Healthcare DICOM Service should be created. Changing this forces a new Healthcare DICOM Service to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Healthcare DICOM Service. Changing this forces a new Healthcare DICOM Service to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        pub private_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::healthcare::DicomServicePrivateEndpoint>,
        >,
        /// Whether to enabled public networks when data plane traffic coming from public networks while private endpoint is enabled. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The url of the Healthcare DICOM Services.
        pub service_url: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Healthcare DICOM Service.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare DICOM Service should exist. Changing this forces a new Healthcare DICOM Service to be created.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DicomServiceArgs) -> DicomServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let tags_binding = args.tags.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/dicomService:DicomService".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authentications".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
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
                    name: "publicNetworkAccessEnabled".into(),
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
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DicomServiceResult {
            authentications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authentications").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
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
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
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
