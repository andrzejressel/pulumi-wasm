/// Models are deployed into it, and afterwards Endpoint is called to obtain predictions and explanations.
///
///
/// To get more information about Endpoint, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/vertex-ai/docs)
///
/// ## Example Usage
///
/// ### Vertex Ai Endpoint Network
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
///       encryptionSpec:
///         kmsKeyName: kms-name
///       predictRequestResponseLoggingConfig:
///         bigqueryDestination:
///           outputUri: bq://${project.projectId}.${bqDataset.datasetId}.request_response_logging
///         enabled: true
///         samplingRate: 0.1
///       trafficSplit:
///         fn::toJSON:
///           '12345': 100
///     options:
///       dependsOn:
///         - ${vertexVpcConnection}
///   vertexVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vertex_vpc_connection
///     properties:
///       network: ${vertexNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${vertexRange.name}
///   vertexRange:
///     type: gcp:compute:GlobalAddress
///     name: vertex_range
///     properties:
///       name: address-name
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 24
///       network: ${vertexNetwork.id}
///   vertexNetwork:
///     type: gcp:compute:Network
///     name: vertex_network
///     properties:
///       name: network-name
///   cryptoKey:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key
///     properties:
///       cryptoKeyId: kms-name
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@gcp-sa-aiplatform.iam.gserviceaccount.com
///   bqDataset:
///     type: gcp:bigquery:Dataset
///     name: bq_dataset
///     properties:
///       datasetId: some_dataset
///       friendlyName: logging dataset
///       description: This is a dataset that requests are logged to
///       location: US
///       deleteContentsOnDestroy: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Endpoint Private Service Connect
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name_8270
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       privateServiceConnectConfig:
///         enablePrivateServiceConnect: true
///         projectAllowlists:
///           - ${project.projectId}
///         enableSecurePrivateServiceConnect: false
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Endpoint Dedicated Endpoint
///
///
/// ```yaml
/// resources:
///   endpoint:
///     type: gcp:vertex:AiEndpoint
///     properties:
///       name: endpoint-name_41150
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       location: us-central1
///       region: us-central1
///       labels:
///         label-one: value-one
///       dedicatedEndpointEnabled: true
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Endpoint can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/endpoints/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Endpoint can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default projects/{{project}}/locations/{{location}}/endpoints/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiEndpoint:AiEndpoint default {{location}}/{{name}}
/// ```
///
pub mod ai_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiEndpointArgs {
        /// If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon.
        #[builder(into, default)]
        pub dedicated_endpoint_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The description of the Endpoint.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key.
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_spec: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::vertex::AiEndpointEncryptionSpec>,
        >,
        /// The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. Only one of the fields, `network` or `privateServiceConnectConfig`, can be set.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configures the request-response logging for online prediction.
        /// Structure is documented below.
        #[builder(into, default)]
        pub predict_request_response_logging_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiEndpointPredictRequestResponseLoggingConfig,
            >,
        >,
        /// Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_service_connect_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::vertex::AiEndpointPrivateServiceConnectConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region for the resource
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.
        /// If a DeployedModel's id is not listed in this map, then it receives no traffic.
        /// The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See
        /// the `deployModel` [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and
        /// [documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.
        /// > **Note:** To set the map to empty, set `"{}"`, apply, and then remove the field from your config.
        #[builder(into, default)]
        pub traffic_split: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AiEndpointResult {
        /// (Output)
        /// Output only. Timestamp when the DeployedModel was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. DNS of the dedicated endpoint. Will only be populated if dedicatedEndpointEnabled is true. Format: `https://{endpointId}.{region}-{projectNumber}.prediction.vertexai.goog`.
        pub dedicated_endpoint_dns: pulumi_wasm_rust::Output<String>,
        /// If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon.
        pub dedicated_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. Models can also be deployed and undeployed using the [Cloud Console](https://console.cloud.google.com/vertex-ai/).
        /// Structure is documented below.
        pub deployed_models: pulumi_wasm_rust::Output<
            Vec<super::super::types::vertex::AiEndpointDeployedModel>,
        >,
        /// The description of the Endpoint.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key.
        /// Structure is documented below.
        pub encryption_spec: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiEndpointEncryptionSpec>,
        >,
        /// Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}`
        pub model_deployment_monitoring_job: pulumi_wasm_rust::Output<String>,
        /// The resource name of the Endpoint. The name must be numeric with no leading zeros and can be at most 10 digits.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. Only one of the fields, `network` or `privateServiceConnectConfig`, can be set.
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// Configures the request-response logging for online prediction.
        /// Structure is documented below.
        pub predict_request_response_logging_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiEndpointPredictRequestResponseLoggingConfig,
            >,
        >,
        /// Configuration for private service connect. `network` and `privateServiceConnectConfig` are mutually exclusive.
        /// Structure is documented below.
        pub private_service_connect_config: pulumi_wasm_rust::Output<
            Option<super::super::types::vertex::AiEndpointPrivateServiceConnectConfig>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The region for the resource
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// A map from a DeployedModel's id to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel.
        /// If a DeployedModel's id is not listed in this map, then it receives no traffic.
        /// The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. See
        /// the `deployModel` [example](https://cloud.google.com/vertex-ai/docs/general/deployment#deploy_a_model_to_an_endpoint) and
        /// [documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.endpoints/deployModel) for more information.
        /// > **Note:** To set the map to empty, set `"{}"`, apply, and then remove the field from your config.
        pub traffic_split: pulumi_wasm_rust::Output<String>,
        /// Output only. Timestamp when this Endpoint was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiEndpointArgs,
    ) -> AiEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dedicated_endpoint_enabled_binding = args
            .dedicated_endpoint_enabled
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let encryption_spec_binding = args
            .encryption_spec
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let predict_request_response_logging_config_binding = args
            .predict_request_response_logging_config
            .get_output(context)
            .get_inner();
        let private_service_connect_config_binding = args
            .private_service_connect_config
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let traffic_split_binding = args.traffic_split.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiEndpoint:AiEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dedicatedEndpointEnabled".into(),
                    value: &dedicated_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionSpec".into(),
                    value: &encryption_spec_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "predictRequestResponseLoggingConfig".into(),
                    value: &predict_request_response_logging_config_binding,
                },
                register_interface::ObjectField {
                    name: "privateServiceConnectConfig".into(),
                    value: &private_service_connect_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "trafficSplit".into(),
                    value: &traffic_split_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedEndpointDns".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedEndpointEnabled".into(),
                },
                register_interface::ResultField {
                    name: "deployedModels".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionSpec".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "modelDeploymentMonitoringJob".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "predictRequestResponseLoggingConfig".into(),
                },
                register_interface::ResultField {
                    name: "privateServiceConnectConfig".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "trafficSplit".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AiEndpointResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dedicated_endpoint_dns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedEndpointDns").unwrap(),
            ),
            dedicated_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedEndpointEnabled").unwrap(),
            ),
            deployed_models: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deployedModels").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionSpec").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            model_deployment_monitoring_job: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modelDeploymentMonitoringJob").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            predict_request_response_logging_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("predictRequestResponseLoggingConfig").unwrap(),
            ),
            private_service_connect_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateServiceConnectConfig").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            traffic_split: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficSplit").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
