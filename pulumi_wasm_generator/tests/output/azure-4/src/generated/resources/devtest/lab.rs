/// Manages a Dev Test Lab.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleLab:
///     type: azure:devtest:Lab
///     name: example
///     properties:
///       name: example-devtestlab
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         Sydney: Australia
/// ```
///
/// ## Import
///
/// Dev Test Labs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devtest/lab:Lab lab1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevTestLab/labs/lab1
/// ```
///
pub mod lab {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LabArgs {
        /// Specifies the supported Azure location where the Dev Test Lab should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Dev Test Lab. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Dev Test Lab resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LabResult {
        /// The ID of the Storage Account used for Artifact Storage.
        pub artifacts_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Default Premium Storage Account for this Dev Test Lab.
        pub default_premium_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Default Storage Account for this Dev Test Lab.
        pub default_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Key used for this Dev Test Lab.
        pub key_vault_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the Dev Test Lab should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Dev Test Lab. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Storage Account used for Storage of Premium Data Disk.
        pub premium_data_disk_storage_account_id: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the Dev Test Lab resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The unique immutable identifier of the Dev Test Lab.
        pub unique_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LabArgs,
    ) -> LabResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devtest/lab:Lab".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "artifactsStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "defaultPremiumStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "defaultStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "premiumDataDiskStorageAccountId".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "uniqueIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LabResult {
            artifacts_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("artifactsStorageAccountId").unwrap(),
            ),
            default_premium_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPremiumStorageAccountId").unwrap(),
            ),
            default_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultStorageAccountId").unwrap(),
            ),
            key_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            premium_data_disk_storage_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("premiumDataDiskStorageAccountId").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            unique_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueIdentifier").unwrap(),
            ),
        }
    }
}
