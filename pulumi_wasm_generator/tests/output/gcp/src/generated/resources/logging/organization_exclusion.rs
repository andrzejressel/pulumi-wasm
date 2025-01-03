/// Manages an organization-level logging exclusion. For more information see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/organizations.exclusions)
/// * How-to Guides
///     * [Excluding Logs](https://cloud.google.com/logging/docs/exclusions)
///
/// > You can specify exclusions for log sinks created by the provider by using the exclusions field of `gcp.logging.OrganizationSink`
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-exclusion:
///     type: gcp:logging:OrganizationExclusion
///     properties:
///       name: my-instance-debug-exclusion
///       orgId: '123456789'
///       description: Exclude GCE instance debug logs
///       filter: resource.type = gce_instance AND severity <= DEBUG
/// ```
///
/// ## Import
///
/// Organization-level logging exclusions can be imported using their URI, e.g.
///
/// * `organizations/{{organization}}/exclusions/{{name}}`
///
/// When using the `pulumi import` command, organization-level logging exclusions can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/organizationExclusion:OrganizationExclusion default organizations/{{organization}}/exclusions/{{name}}
/// ```
///
pub mod organization_exclusion {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationExclusionArgs {
        /// A human-readable description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        #[builder(into)]
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The name of the logging exclusion.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The organization to create the exclusion in.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationExclusionResult {
        /// A human-readable description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether this exclusion rule should be disabled or not. This defaults to
        /// false.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The filter to apply when excluding logs. Only log entries that match the filter are excluded.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced-filters) for information on how to
        /// write a filter.
        pub filter: pulumi_wasm_rust::Output<String>,
        /// The name of the logging exclusion.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The organization to create the exclusion in.
        pub org_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OrganizationExclusionArgs,
    ) -> OrganizationExclusionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let disabled_binding = args.disabled.get_inner();
        let filter_binding = args.filter.get_inner();
        let name_binding = args.name.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/organizationExclusion:OrganizationExclusion".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OrganizationExclusionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
        }
    }
}
