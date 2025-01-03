/// A managed metastore federation.
///
///
///
/// ## Example Usage
///
/// ### Dataproc Metastore Federation Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = metastore_federation::create(
///         "default",
///         MetastoreFederationArgs::builder()
///             .backend_metastores(
///                 vec![
///                     MetastoreFederationBackendMetastore::builder()
///                     .metastoreType("DATAPROC_METASTORE")
///                     .name("${defaultMetastoreService.id}").rank("1").build_struct(),
///                 ],
///             )
///             .federation_id("metastore-fed")
///             .location("us-central1")
///             .version("3.1.2")
///             .build_struct(),
///     );
///     let defaultMetastoreService = metastore_service::create(
///         "defaultMetastoreService",
///         MetastoreServiceArgs::builder()
///             .hive_metastore_config(
///                 MetastoreServiceHiveMetastoreConfig::builder()
///                     .endpointProtocol("GRPC")
///                     .version("3.1.2")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .service_id("metastore-service")
///             .tier("DEVELOPER")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dataproc Metastore Federation Bigquery
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:dataproc:MetastoreFederation
///     properties:
///       location: us-central1
///       federationId: metastore-fed
///       version: 3.1.2
///       backendMetastores:
///         - rank: '2'
///           name: ${project.id}
///           metastoreType: BIGQUERY
///         - rank: '1'
///           name: ${defaultMetastoreService.id}
///           metastoreType: DATAPROC_METASTORE
///   defaultMetastoreService:
///     type: gcp:dataproc:MetastoreService
///     name: default
///     properties:
///       serviceId: metastore-service
///       location: us-central1
///       tier: DEVELOPER
///       hiveMetastoreConfig:
///         version: 3.1.2
///         endpointProtocol: GRPC
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Federation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/federations/{{federation_id}}`
///
/// * `{{project}}/{{location}}/{{federation_id}}`
///
/// * `{{location}}/{{federation_id}}`
///
/// When using the `pulumi import` command, Federation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreFederation:MetastoreFederation default projects/{{project}}/locations/{{location}}/federations/{{federation_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreFederation:MetastoreFederation default {{project}}/{{location}}/{{federation_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/metastoreFederation:MetastoreFederation default {{location}}/{{federation_id}}
/// ```
///
pub mod metastore_federation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetastoreFederationArgs {
        /// A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number.
        /// Structure is documented below.
        #[builder(into)]
        pub backend_metastores: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataproc::MetastoreFederationBackendMetastore>,
        >,
        /// The ID of the metastore federation. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 63 characters.
        #[builder(into)]
        pub federation_id: pulumi_wasm_rust::Output<String>,
        /// User-defined labels for the metastore federation. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the metastore federation should reside.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version.
        #[builder(into)]
        pub version: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MetastoreFederationResult {
        /// A map from BackendMetastore rank to BackendMetastores from which the federation service serves metadata at query time. The map key represents the order in which BackendMetastores should be evaluated to resolve database names at query time and should be greater than or equal to zero. A BackendMetastore with a lower number will be evaluated before a BackendMetastore with a higher number.
        /// Structure is documented below.
        pub backend_metastores: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataproc::MetastoreFederationBackendMetastore>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URI of the endpoint used to access the metastore federation.
        pub endpoint_uri: pulumi_wasm_rust::Output<String>,
        /// The ID of the metastore federation. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),
        /// and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between
        /// 3 and 63 characters.
        pub federation_id: pulumi_wasm_rust::Output<String>,
        /// User-defined labels for the metastore federation. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the metastore federation should reside.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The relative resource name of the metastore federation.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current state of the metastore federation.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Additional information about the current state of the metastore federation, if available.
        pub state_message: pulumi_wasm_rust::Output<String>,
        /// The globally unique resource identifier of the metastore federation.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The Apache Hive metastore version of the federation. All backend metastore versions must be compatible with the federation version.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MetastoreFederationArgs,
    ) -> MetastoreFederationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backend_metastores_binding = args.backend_metastores.get_inner();
        let federation_id_binding = args.federation_id.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/metastoreFederation:MetastoreFederation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backendMetastores".into(),
                    value: &backend_metastores_binding,
                },
                register_interface::ObjectField {
                    name: "federationId".into(),
                    value: &federation_id_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "backendMetastores".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "endpointUri".into(),
                },
                register_interface::ResultField {
                    name: "federationId".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
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
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateMessage".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MetastoreFederationResult {
            backend_metastores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backendMetastores").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            endpoint_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUri").unwrap(),
            ),
            federation_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("federationId").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
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
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateMessage").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
