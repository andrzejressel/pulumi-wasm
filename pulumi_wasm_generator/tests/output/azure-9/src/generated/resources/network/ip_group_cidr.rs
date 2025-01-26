/// Manages IP Group CIDR records.
///
/// > Warning Do not use this resource at the same time as the `cidrs` property of the
/// `azure.network.IPGroup` resource for the same IP Group. Doing so will cause a conflict and
/// CIDRS will be removed.
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
///             .name("test-rg")
///             .build_struct(),
///     );
///     let exampleIPGroup = ip_group::create(
///         "exampleIPGroup",
///         IpGroupArgs::builder()
///             .location("${example.location}")
///             .name("test-ipgroup")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleIPGroupCIDR = ip_group_cidr::create(
///         "exampleIPGroupCIDR",
///         IpGroupCidrArgs::builder()
///             .cidr("10.10.10.0/24")
///             .ip_group_id("${exampleIPGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IP Group CIDRs can be imported using the `resource id` of the IP Group and
///
/// the CIDR value (`/` characters have to be replaced by `_`), e.g.
///
/// ```sh
/// $ pulumi import azure:network/iPGroupCIDR:IPGroupCIDR example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/test-rg/providers/Microsoft.Network/ipGroups/test-ipgroup/cidrs/10.1.0.0_24
/// ```
///
pub mod ip_group_cidr {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IPGroupCIDRArgs {
        #[builder(into)]
        pub cidr: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the destination IP Group.
        /// Changing this forces a new IP Group CIDR to be created.
        #[builder(into)]
        pub ip_group_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IPGroupCIDRResult {
        pub cidr: pulumi_wasm_rust::Output<String>,
        /// The ID of the destination IP Group.
        /// Changing this forces a new IP Group CIDR to be created.
        pub ip_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IPGroupCIDRArgs,
    ) -> IPGroupCIDRResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cidr_binding = args.cidr.get_output(context).get_inner();
        let ip_group_id_binding = args.ip_group_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/iPGroupCIDR:IPGroupCIDR".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "ipGroupId".into(),
                    value: &ip_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cidr".into(),
                },
                register_interface::ResultField {
                    name: "ipGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IPGroupCIDRResult {
            cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidr").unwrap(),
            ),
            ip_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipGroupId").unwrap(),
            ),
        }
    }
}
