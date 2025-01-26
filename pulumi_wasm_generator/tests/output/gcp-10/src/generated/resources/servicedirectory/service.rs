/// An individual service. A service contains a name and optional metadata.
///
/// To get more information about Service, see:
///
/// * [API documentation](https://cloud.google.com/service-directory/docs/reference/rest/v1beta1/projects.locations.namespaces.services)
/// * How-to Guides
///     * [Configuring a service](https://cloud.google.com/service-directory/docs/configuring-service-directory#configuring_a_service)
///
/// ## Example Usage
///
/// ### Service Directory Service Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:servicedirectory:Namespace
///     properties:
///       namespaceId: example-namespace
///       location: us-central1
///   exampleService:
///     type: gcp:servicedirectory:Service
///     name: example
///     properties:
///       serviceId: example-service
///       namespace: ${example.id}
///       metadata:
///         stage: prod
///         region: us-central1
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}/services/{{service_id}}`
///
/// * `{{project}}/{{location}}/{{namespace_id}}/{{service_id}}`
///
/// * `{{location}}/{{namespace_id}}/{{service_id}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/service:Service default projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}/services/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/service:Service default {{project}}/{{location}}/{{namespace_id}}/{{service_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/service:Service default {{location}}/{{namespace_id}}/{{service_id}}
/// ```
///
pub mod service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// Metadata for the service. This data can be consumed
        /// by service clients. The entire metadata dictionary may contain
        /// up to 2000 characters, spread across all key-value pairs.
        /// Metadata that goes beyond any these limits will be rejected.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name of the namespace this service will belong to.
        #[builder(into)]
        pub namespace: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Resource ID must be 1-63 characters long, including digits,
        /// lowercase letters or the hyphen character.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// Metadata for the service. This data can be consumed
        /// by service clients. The entire metadata dictionary may contain
        /// up to 2000 characters, spread across all key-value pairs.
        /// Metadata that goes beyond any these limits will be rejected.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The resource name for the service in the
        /// format `projects/*/locations/*/namespaces/*/services/*`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the namespace this service will belong to.
        pub namespace: pulumi_wasm_rust::Output<String>,
        /// The Resource ID must be 1-63 characters long, including digits,
        /// lowercase letters or the hyphen character.
        ///
        ///
        /// - - -
        pub service_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let namespace_binding = args.namespace.get_output(context).get_inner();
        let service_id_binding = args.service_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:servicedirectory/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "namespace".into(),
                    value: &namespace_binding,
                },
                register_interface::ObjectField {
                    name: "serviceId".into(),
                    value: &service_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            namespace: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namespace"),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
        }
    }
}
