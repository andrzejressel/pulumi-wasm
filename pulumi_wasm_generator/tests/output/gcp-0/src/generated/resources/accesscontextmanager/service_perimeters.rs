/// Replace all existing Service Perimeters in an Access Policy with the Service Perimeters provided. This is done atomically.
/// This is a bulk edit of all Service Perimeters and may override existing Service Perimeters created by `gcp.accesscontextmanager.ServicePerimeter`,
/// thus causing a permadiff if used alongside `gcp.accesscontextmanager.ServicePerimeter` on the same parent.
///
///
/// To get more information about ServicePerimeters, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters)
/// * How-to Guides
///     * [Guide to Ingress and Egress Rules](https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules)
///     * [Service Perimeter Quickstart](https://cloud.google.com/vpc-service-controls/docs/quickstart)
///
/// ## Example Usage
///
/// ### Access Context Manager Service Perimeters Basic
///
///
/// ```yaml
/// resources:
///   service-perimeter:
///     type: gcp:accesscontextmanager:ServicePerimeters
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       servicePerimeters:
///         - name: accessPolicies/${["access-policy"].name}/servicePerimeters/
///           title: ""
///           status:
///             restrictedServices:
///               - storage.googleapis.com
///         - name: accessPolicies/${["access-policy"].name}/servicePerimeters/
///           title: ""
///           status:
///             restrictedServices:
///               - bigtable.googleapis.com
///   access-level:
///     type: gcp:accesscontextmanager:AccessLevel
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/accessLevels/chromeos_no_lock
///       title: chromeos_no_lock
///       basic:
///         conditions:
///           - devicePolicy:
///               requireScreenLock: false
///               osConstraints:
///                 - osType: DESKTOP_CHROME_OS
///             regions:
///               - CH
///               - IT
///               - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// ServicePerimeters can be imported using any of these accepted formats:
///
/// * `{{parent}}/servicePerimeters`
///
/// * `{{parent}}`
///
/// When using the `pulumi import` command, ServicePerimeters can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/servicePerimeters:ServicePerimeters default {{parent}}/servicePerimeters
/// ```
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/servicePerimeters:ServicePerimeters default {{parent}}
/// ```
///
pub mod service_perimeters {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimetersArgs {
        /// The AccessPolicy this ServicePerimeter lives in.
        /// Format: accessPolicies/{policy_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The desired Service Perimeters that should replace all existing Service Perimeters in the Access Policy.
        /// Structure is documented below.
        #[builder(into, default)]
        pub service_perimeters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::accesscontextmanager::ServicePerimetersServicePerimeter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ServicePerimetersResult {
        /// The AccessPolicy this ServicePerimeter lives in.
        /// Format: accessPolicies/{policy_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
        /// The desired Service Perimeters that should replace all existing Service Perimeters in the Access Policy.
        /// Structure is documented below.
        pub service_perimeters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::accesscontextmanager::ServicePerimetersServicePerimeter,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServicePerimetersArgs) -> ServicePerimetersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let parent_binding = args.parent.get_inner();
        let service_perimeters_binding = args.service_perimeters.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeters:ServicePerimeters".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "servicePerimeters".into(),
                    value: &service_perimeters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "servicePerimeters".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServicePerimetersResult {
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            service_perimeters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePerimeters").unwrap(),
            ),
        }
    }
}
