/// Hierarchical firewall policy rules let you create and enforce a consistent firewall policy across your organization. Rules can explicitly allow or deny connections or delegate evaluation to lower level policies. Policies can be created within organizations or folders.
///
/// This resource should be generally be used with `gcp.compute.FirewallPolicyAssociation` and `gcp.compute.FirewallPolicyRule`
///
/// For more information see the [official documentation](https://cloud.google.com/vpc/docs/firewall-policies)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = firewall_policy::create(
///         "default",
///         FirewallPolicyArgs::builder()
///             .description("Example Resource")
///             .parent("organizations/12345")
///             .short_name("my-policy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FirewallPolicy can be imported using any of these accepted formats:
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicy:FirewallPolicy default locations/global/firewallPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/firewallPolicy:FirewallPolicy default {{name}}
/// ```
///
pub mod firewall_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FirewallPolicyArgs {
        /// An optional description of this resource. Provide this property when you create the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The parent of the firewall policy.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
        /// User-provided name of the Organization firewall policy. The name should be unique in the organization in which the firewall policy is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        ///
        /// - - -
        #[builder(into)]
        pub short_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FirewallPolicyResult {
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource. Provide this property when you create the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Fingerprint of the resource. This field is used internally during updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub firewall_policy_id: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. It is a numeric ID allocated by GCP which uniquely identifies the Firewall Policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parent of the firewall policy.
        pub parent: pulumi_wasm_rust::Output<String>,
        /// Total count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples.
        pub rule_tuple_count: pulumi_wasm_rust::Output<i32>,
        /// Server-defined URL for the resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Server-defined URL for this resource with the resource id.
        pub self_link_with_id: pulumi_wasm_rust::Output<String>,
        /// User-provided name of the Organization firewall policy. The name should be unique in the organization in which the firewall policy is created. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression a-z? which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
        ///
        ///
        ///
        /// - - -
        pub short_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FirewallPolicyArgs,
    ) -> FirewallPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let short_name_binding = args.short_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/firewallPolicy:FirewallPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "shortName".into(),
                    value: &short_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FirewallPolicyResult {
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fingerprint"),
            ),
            firewall_policy_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firewallPolicyId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            rule_tuple_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ruleTupleCount"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            self_link_with_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLinkWithId"),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortName"),
            ),
        }
    }
}
