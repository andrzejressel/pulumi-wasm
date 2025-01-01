/// AuthzExtension is a resource that allows traffic forwarding to a callout backend service to make an authorization decision.
///
///
/// To get more information about AuthzExtension, see:
///
/// * [API documentation](https://cloud.google.com/service-extensions/docs/reference/rest/v1beta1/projects.locations.authzExtensions)
///
/// ## Example Usage
///
/// ### Network Services Authz Extension Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = region_backend_service::create(
///         "default",
///         RegionBackendServiceArgs::builder()
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .name("authz-service")
///             .port_name("grpc")
///             .project("my-project-name")
///             .protocol("HTTP2")
///             .region("us-west1")
///             .build_struct(),
///     );
///     let defaultAuthzExtension = authz_extension::create(
///         "defaultAuthzExtension",
///         AuthzExtensionArgs::builder()
///             .authority("ext11.com")
///             .description("my description")
///             .fail_open(false)
///             .forward_headers(vec!["Authorization",])
///             .load_balancing_scheme("INTERNAL_MANAGED")
///             .location("us-west1")
///             .name("my-authz-ext")
///             .project("my-project-name")
///             .service("${default.selfLink}")
///             .timeout("0.1s")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// AuthzExtension can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/authzExtensions/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AuthzExtension can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/authzExtension:AuthzExtension default projects/{{project}}/locations/{{location}}/authzExtensions/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/authzExtension:AuthzExtension default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/authzExtension:AuthzExtension default {{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/authzExtension:AuthzExtension default {{name}}
/// ```
///
pub mod authz_extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthzExtensionArgs {
        /// The :authority header in the gRPC request sent from Envoy to the extension service.
        #[builder(into)]
        pub authority: pulumi_wasm_rust::Output<String>,
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Determines how the proxy behaves if the call to the extension fails or times out.
        /// When set to TRUE, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to FALSE or the default setting of FALSE is used, one of the following happens:
        /// * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer.
        /// * If response headers have been delivered, then the HTTP stream to the downstream client is reset.
        #[builder(into, default)]
        pub fail_open: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name.
        #[builder(into, default)]
        pub forward_headers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of labels associated with the AuthzExtension resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.
        /// For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        #[builder(into)]
        pub load_balancing_scheme: pulumi_wasm_rust::Output<String>,
        /// The location of the resource.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The metadata provided here is included as part of the metadata_context (of type google.protobuf.Struct) in the ProcessingRequest message sent to the extension server. The metadata is available under the namespace com.google.authz_extension.<resourceName>. The following variables are supported in the metadata Struct:
        /// {forwarding_rule_id} - substituted with the forwarding rule's fully qualified resource name.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier. Name of the AuthzExtension resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The reference to the service that runs the extension.
        /// To configure a callout extension, service must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format:
        /// https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService} or https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}.
        #[builder(into)]
        pub service: pulumi_wasm_rust::Output<String>,
        /// Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds.
        #[builder(into)]
        pub timeout: pulumi_wasm_rust::Output<String>,
        /// The format of communication supported by the callout extension.
        /// Default value is `EXT_PROC_GRPC`.
        /// Possible values are: `WIRE_FORMAT_UNSPECIFIED`, `EXT_PROC_GRPC`.
        #[builder(into, default)]
        pub wire_format: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthzExtensionResult {
        /// The :authority header in the gRPC request sent from Envoy to the extension service.
        pub authority: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A human-readable description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Determines how the proxy behaves if the call to the extension fails or times out.
        /// When set to TRUE, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to FALSE or the default setting of FALSE is used, one of the following happens:
        /// * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer.
        /// * If response headers have been delivered, then the HTTP stream to the downstream client is reset.
        pub fail_open: pulumi_wasm_rust::Output<bool>,
        /// List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name.
        pub forward_headers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of labels associated with the AuthzExtension resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// All backend services and forwarding rules referenced by this extension must share the same load balancing scheme.
        /// For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service).
        /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`.
        pub load_balancing_scheme: pulumi_wasm_rust::Output<String>,
        /// The location of the resource.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The metadata provided here is included as part of the metadata_context (of type google.protobuf.Struct) in the ProcessingRequest message sent to the extension server. The metadata is available under the namespace com.google.authz_extension.<resourceName>. The following variables are supported in the metadata Struct:
        /// {forwarding_rule_id} - substituted with the forwarding rule's fully qualified resource name.
        pub metadata: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier. Name of the AuthzExtension resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The reference to the service that runs the extension.
        /// To configure a callout extension, service must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format:
        /// https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService} or https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}.
        pub service: pulumi_wasm_rust::Output<String>,
        /// Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds.
        pub timeout: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// The format of communication supported by the callout extension.
        /// Default value is `EXT_PROC_GRPC`.
        /// Possible values are: `WIRE_FORMAT_UNSPECIFIED`, `EXT_PROC_GRPC`.
        pub wire_format: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AuthzExtensionArgs) -> AuthzExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authority_binding = args.authority.get_inner();
        let description_binding = args.description.get_inner();
        let fail_open_binding = args.fail_open.get_inner();
        let forward_headers_binding = args.forward_headers.get_inner();
        let labels_binding = args.labels.get_inner();
        let load_balancing_scheme_binding = args.load_balancing_scheme.get_inner();
        let location_binding = args.location.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let service_binding = args.service.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let wire_format_binding = args.wire_format.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/authzExtension:AuthzExtension".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authority".into(),
                    value: &authority_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "failOpen".into(),
                    value: &fail_open_binding,
                },
                register_interface::ObjectField {
                    name: "forwardHeaders".into(),
                    value: &forward_headers_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancingScheme".into(),
                    value: &load_balancing_scheme_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "service".into(),
                    value: &service_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "wireFormat".into(),
                    value: &wire_format_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "authority".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "failOpen".into(),
                },
                register_interface::ResultField {
                    name: "forwardHeaders".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancingScheme".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "service".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "wireFormat".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AuthzExtensionResult {
            authority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authority").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            fail_open: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failOpen").unwrap(),
            ),
            forward_headers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardHeaders").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            load_balancing_scheme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancingScheme").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("service").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            wire_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("wireFormat").unwrap(),
            ),
        }
    }
}
