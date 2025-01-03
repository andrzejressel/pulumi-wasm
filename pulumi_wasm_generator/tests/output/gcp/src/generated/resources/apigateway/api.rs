/// A consumable API that can be used by multiple Gateways.
///
/// To get more information about Api, see:
///
/// * [API documentation](https://cloud.google.com/api-gateway/docs/reference/rest/v1beta/projects.locations.apis)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/api-gateway/docs/quickstart)
///
/// ## Example Usage
///
/// ### Apigateway Api Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let api = api::create("api", ApiArgs::builder().api_id("my-api").build_struct());
/// }
/// ```
///
/// ## Import
///
/// Api can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/apis/{{api_id}}`
///
/// * `{{project}}/{{api_id}}`
///
/// * `{{api_id}}`
///
/// When using the `pulumi import` command, Api can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default projects/{{project}}/locations/global/apis/{{api_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default {{project}}/{{api_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigateway/api:Api default {{api_id}}
/// ```
///
pub mod api {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiArgs {
        /// Identifier to assign to the API. Must be unique within scope of the parent resource(project)
        ///
        ///
        /// - - -
        #[builder(into)]
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// A user-visible name for the API.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed).
        /// If not specified, a new Service will automatically be created in the same project as this API.
        #[builder(into, default)]
        pub managed_service: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ApiResult {
        /// Identifier to assign to the API. Must be unique within scope of the parent resource(project)
        ///
        ///
        /// - - -
        pub api_id: pulumi_wasm_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A user-visible name for the API.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user-provided metadata.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed).
        /// If not specified, a new Service will automatically be created in the same project as this API.
        pub managed_service: pulumi_wasm_rust::Output<String>,
        /// The resource name of the API. Format `projects/{{project}}/locations/global/apis/{{apiId}}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApiArgs) -> ApiResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_id_binding = args.api_id.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let managed_service_binding = args.managed_service.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigateway/api:Api".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiId".into(),
                    value: &api_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "managedService".into(),
                    value: &managed_service_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "managedService".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApiResult {
            api_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            managed_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedService").unwrap(),
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
        }
    }
}
