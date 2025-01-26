/// Manages an IP group that contains a list of CIDRs and/or IP addresses.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleIPGroup:
///     type: azure:network:IPGroup
///     name: example
///     properties:
///       name: example1-ipgroup
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       cidrs:
///         - 192.168.0.1
///         - 172.16.240.0/20
///         - 10.48.0.0/12
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// IP Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/iPGroup:IPGroup ipgroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/ipGroups/myIpGroup
/// ```
///
pub mod ip_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IPGroupArgs {
        #[builder(into, default)]
        pub cidrs: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the IP group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the IP group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IPGroupResult {
        pub cidrs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A list of ID of Firewall.
        pub firewall_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of ID of Firewall Policy`.
        pub firewall_policy_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the IP group. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the IP group. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IPGroupArgs,
    ) -> IPGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidrs_binding = args.cidrs.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/iPGroup:IPGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidrs".into(),
                    value: &cidrs_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IPGroupResult {
            cidrs: pulumi_wasm_rust::__private::into_domain(o.extract_field("cidrs")),
            firewall_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallIds"),
            ),
            firewall_policy_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallPolicyIds"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
