/// Organization security policies are used to control incoming/outgoing traffic.
///
/// To get more information about OrganizationSecurityPolicy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/organizationSecurityPolicies)
/// * How-to Guides
///     * [Creating a firewall policy](https://cloud.google.com/vpc/docs/using-firewall-policies#create-policy)
///
/// ## Example Usage
///
/// ### Organization Security Policy Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = organization_security_policy::create(
///         "policy",
///         OrganizationSecurityPolicyArgs::builder()
///             .display_name("tf-test")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationSecurityPolicy can be imported using any of these accepted formats:
///
/// * `locations/global/securityPolicies/{{policy_id}}`
///
/// * `{{policy_id}}`
///
/// When using the `pulumi import` command, OrganizationSecurityPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy default locations/global/securityPolicies/{{policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy default {{policy_id}}
/// ```
///
pub mod organization_security_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyArgs {
        /// A textual description for the organization security policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A textual name of the security policy.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The type indicates the intended use of the security policy.
        /// For organization security policies, the only supported type
        /// is "FIREWALL".
        /// Default value is `FIREWALL`.
        /// Possible values are: `FIREWALL`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyResult {
        /// A textual description for the organization security policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A textual name of the security policy.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub policy_id: pulumi_wasm_rust::Output<String>,
        /// The type indicates the intended use of the security policy.
        /// For organization security policies, the only supported type
        /// is "FIREWALL".
        /// Default value is `FIREWALL`.
        /// Possible values are: `FIREWALL`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationSecurityPolicyArgs,
    ) -> OrganizationSecurityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let parent_binding = args.parent.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "policyId".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationSecurityPolicyResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            policy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyId").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
