/// Manages a PowerBI Embedded.
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
///     let exampleEmbedded = embedded::create(
///         "exampleEmbedded",
///         EmbeddedArgs::builder()
///             .administrators(vec!["azsdktest@microsoft.com",])
///             .location("${example.location}")
///             .name("examplepowerbi")
///             .resource_group_name("${example.name}")
///             .sku_name("A1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PowerBI Embedded can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:powerbi/embedded:Embedded example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.PowerBIDedicated/capacities/capacity1
/// ```
///
pub mod embedded {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmbeddedArgs {
        /// A set of administrator user identities, which manages the Power BI Embedded and must be a member user or a service principal in your AAD tenant.
        #[builder(into)]
        pub administrators: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Sets the PowerBI Embedded's mode. Possible values include: `Gen1`, `Gen2`. Defaults to `Gen1`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the PowerBI Embedded. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the PowerBI Embedded should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Sets the PowerBI Embedded's pricing level's SKU. Possible values include: `A1`, `A2`, `A3`, `A4`, `A5`, `A6`.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EmbeddedResult {
        /// A set of administrator user identities, which manages the Power BI Embedded and must be a member user or a service principal in your AAD tenant.
        pub administrators: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Sets the PowerBI Embedded's mode. Possible values include: `Gen1`, `Gen2`. Defaults to `Gen1`. Changing this forces a new resource to be created.
        pub mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the PowerBI Embedded. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the PowerBI Embedded should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Sets the PowerBI Embedded's pricing level's SKU. Possible values include: `A1`, `A2`, `A3`, `A4`, `A5`, `A6`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmbeddedArgs) -> EmbeddedResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrators_binding = args.administrators.get_inner();
        let location_binding = args.location.get_inner();
        let mode_binding = args.mode.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:powerbi/embedded:Embedded".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administrators".into(),
                    value: &administrators_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
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
                    name: "administrators".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
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
        EmbeddedResult {
            administrators: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administrators").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
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