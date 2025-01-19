/// Manages a Front Door (standard/premium) Profile which contains a collection of endpoints and origin groups.
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
///   exampleFrontdoorProfile:
///     type: azure:cdn:FrontdoorProfile
///     name: example
///     properties:
///       name: example-cdn-profile
///       resourceGroupName: ${example.name}
///       skuName: Standard_AzureFrontDoor
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Front Door Profiles can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cdn/frontdoorProfile:FrontdoorProfile example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Cdn/profiles/myprofile1
/// ```
///
pub mod frontdoor_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrontdoorProfileArgs {
        /// Specifies the name of the Front Door Profile. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where this Front Door Profile should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum response timeout in seconds. Possible values are between `16` and `240` seconds (inclusive). Defaults to `120` seconds.
        #[builder(into, default)]
        pub response_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the SKU for this Front Door Profile. Possible values include `Standard_AzureFrontDoor` and `Premium_AzureFrontDoor`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies a mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrontdoorProfileResult {
        /// Specifies the name of the Front Door Profile. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where this Front Door Profile should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The UUID of this Front Door Profile which will be sent in the HTTP Header as the `X-Azure-FDID` attribute.
        pub resource_guid: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum response timeout in seconds. Possible values are between `16` and `240` seconds (inclusive). Defaults to `120` seconds.
        pub response_timeout_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the SKU for this Front Door Profile. Possible values include `Standard_AzureFrontDoor` and `Premium_AzureFrontDoor`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies a mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FrontdoorProfileArgs) -> FrontdoorProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let response_timeout_seconds_binding = args.response_timeout_seconds.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cdn/frontdoorProfile:FrontdoorProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "responseTimeoutSeconds".into(),
                    value: &response_timeout_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGuid".into(),
                },
                register_interface::ResultField {
                    name: "responseTimeoutSeconds".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrontdoorProfileResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            resource_guid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGuid").unwrap(),
            ),
            response_timeout_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("responseTimeoutSeconds").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
