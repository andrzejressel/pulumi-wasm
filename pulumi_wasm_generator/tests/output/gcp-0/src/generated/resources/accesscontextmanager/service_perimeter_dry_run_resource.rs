/// Allows configuring a single GCP resource that should be inside of the `spec` block of a dry run service perimeter.
/// This resource is intended to be used in cases where it is not possible to compile a full list
/// of projects to include in a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// to enable them to be added separately.
/// If your perimeter is NOT in dry-run mode use `gcp.accesscontextmanager.ServicePerimeterResource` instead.
///
/// > **Note:** If this resource is used alongside a `gcp.accesscontextmanager.ServicePerimeter` resource,
/// the service perimeter resource must have a `lifecycle` block with `ignore_changes = [spec[0].resources]` so
/// they don't fight over which resources should be in the policy.
///
///
/// To get more information about ServicePerimeterDryRunResource, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters)
/// * How-to Guides
///     * [Service Perimeter Quickstart](https://cloud.google.com/vpc-service-controls/docs/quickstart)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Service Perimeter Dry Run Resource Basic
///
///
/// ```yaml
/// resources:
///   service-perimeter-dry-run-resource:
///     type: gcp:accesscontextmanager:ServicePerimeterDryRunResource
///     properties:
///       perimeterName: ${["service-perimeter-dry-run-resourceServicePerimeter"].name}
///       resource: projects/987654321
///   service-perimeter-dry-run-resourceServicePerimeter:
///     type: gcp:accesscontextmanager:ServicePerimeter
///     name: service-perimeter-dry-run-resource
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/servicePerimeters/restrict_all
///       title: restrict_all
///       spec:
///         restrictedServices:
///           - storage.googleapis.com
///       useExplicitDryRunSpec: true
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// ServicePerimeterDryRunResource can be imported using any of these accepted formats:
///
/// * `{{perimeter_name}}/{{resource}}`
///
/// When using the `pulumi import` command, ServicePerimeterDryRunResource can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/servicePerimeterDryRunResource:ServicePerimeterDryRunResource default {{perimeter_name}}/{{resource}}
/// ```
///
pub mod service_perimeter_dry_run_resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterDryRunResourceArgs {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub perimeter_name: pulumi_wasm_rust::Output<String>,
        /// A GCP resource that is inside of the service perimeter.
        /// Currently only projects are allowed.
        /// Format: projects/{project_number}
        #[builder(into)]
        pub resource: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ServicePerimeterDryRunResourceResult {
        /// The name of the Service Perimeter to add this resource to.
        ///
        ///
        /// - - -
        pub perimeter_name: pulumi_wasm_rust::Output<String>,
        /// A GCP resource that is inside of the service perimeter.
        /// Currently only projects are allowed.
        /// Format: projects/{project_number}
        pub resource: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServicePerimeterDryRunResourceArgs,
    ) -> ServicePerimeterDryRunResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let perimeter_name_binding = args.perimeter_name.get_inner();
        let resource_binding = args.resource.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeterDryRunResource:ServicePerimeterDryRunResource"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "perimeterName".into(),
                    value: &perimeter_name_binding,
                },
                register_interface::ObjectField {
                    name: "resource".into(),
                    value: &resource_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "perimeterName".into(),
                },
                register_interface::ResultField {
                    name: "resource".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServicePerimeterDryRunResourceResult {
            perimeter_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("perimeterName").unwrap(),
            ),
            resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resource").unwrap(),
            ),
        }
    }
}
