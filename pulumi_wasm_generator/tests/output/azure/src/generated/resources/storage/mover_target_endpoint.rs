/// Manages a Storage Mover Target Endpoint.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .allow_nested_items_to_be_public(true)
///             .location("${example.location}")
///             .name("examplestr")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("blob")
///             .name("example-sc")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleMover = mover::create(
///         "exampleMover",
///         MoverArgs::builder()
///             .location("West Europe")
///             .name("example-ssm")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleMoverTargetEndpoint = mover_target_endpoint::create(
///         "exampleMoverTargetEndpoint",
///         MoverTargetEndpointArgs::builder()
///             .description("Example Storage Container Endpoint Description")
///             .name("example-se")
///             .storage_account_id("${exampleAccount.id}")
///             .storage_container_name("${exampleContainer.name}")
///             .storage_mover_id("${exampleMover.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Storage Mover Target Endpoint can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:storage/moverTargetEndpoint:MoverTargetEndpoint example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.StorageMover/storageMovers/storageMover1/endpoints/endpoint1
/// ```
///
pub mod mover_target_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MoverTargetEndpointArgs {
        /// Specifies a description for the Storage Mover Target Endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the ID of the storage account for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the storage blob container for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage mover for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MoverTargetEndpointResult {
        /// Specifies a description for the Storage Mover Target Endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage account for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub storage_account_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the storage blob container for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the ID of the storage mover for this Storage Mover Target Endpoint. Changing this forces a new resource to be created.
        pub storage_mover_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MoverTargetEndpointArgs,
    ) -> MoverTargetEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let storage_account_id_binding = args.storage_account_id.get_inner();
        let storage_container_name_binding = args.storage_container_name.get_inner();
        let storage_mover_id_binding = args.storage_mover_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:storage/moverTargetEndpoint:MoverTargetEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountId".into(),
                    value: &storage_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerName".into(),
                    value: &storage_container_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageMoverId".into(),
                    value: &storage_mover_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerName".into(),
                },
                register_interface::ResultField {
                    name: "storageMoverId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MoverTargetEndpointResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountId").unwrap(),
            ),
            storage_container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerName").unwrap(),
            ),
            storage_mover_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMoverId").unwrap(),
            ),
        }
    }
}