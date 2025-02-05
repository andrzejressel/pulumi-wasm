/// An endpoint indexes are deployed into. An index endpoint can have multiple deployed indexes.
///
///
/// To get more information about IndexEndpointDeployedIndex, see:
///
/// * [API documentation](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex)
///
/// ## Example Usage
///
/// ### Vertex Ai Index Endpoint Deployed Index Basic
///
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: vertex-sa
///   basicDeployedIndex:
///     type: gcp:vertex:AiIndexEndpointDeployedIndex
///     name: basic_deployed_index
///     properties:
///       indexEndpoint: ${vertexIndexEndpointDeployed.id}
///       index: ${index.id}
///       deployedIndexId: deployed_index_id
///       reservedIpRanges:
///         - vertex-ai-range
///       enableAccessLogging: false
///       displayName: vertex-deployed-index
///       deployedIndexAuthConfig:
///         authProvider:
///           audiences:
///             - 123456-my-app
///           allowedIssuers:
///             - ${sa.email}
///     options:
///       dependsOn:
///         - ${vertexIndexEndpointDeployed}
///         - ${sa}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: bucket-name
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # The sample data comes from the following link:
///   # https://cloud.google.com/vertex-ai/docs/matching-engine/filtering#specify-namespaces-tokens
///   data:
///     type: gcp:storage:BucketObject
///     properties:
///       name: contents/data.json
///       bucket: ${bucket.name}
///       content: |
///         {"id": "42", "embedding": [0.5, 1.0], "restricts": [{"namespace": "class", "allow": ["cat", "pet"]},{"namespace": "category", "allow": ["feline"]}]}
///         {"id": "43", "embedding": [0.6, 1.0], "restricts": [{"namespace": "class", "allow": ["dog", "pet"]},{"namespace": "category", "allow": ["canine"]}]}
///   index:
///     type: gcp:vertex:AiIndex
///     properties:
///       labels:
///         foo: bar
///       region: us-central1
///       displayName: test-index
///       description: index for test
///       metadata:
///         contentsDeltaUri: gs://${bucket.name}/contents
///         config:
///           dimensions: 2
///           approximateNeighborsCount: 150
///           shardSize: SHARD_SIZE_SMALL
///           distanceMeasureType: DOT_PRODUCT_DISTANCE
///           algorithmConfig:
///             treeAhConfig:
///               leafNodeEmbeddingCount: 500
///               leafNodesToSearchPercent: 7
///       indexUpdateMethod: BATCH_UPDATE
///   vertexIndexEndpointDeployed:
///     type: gcp:vertex:AiIndexEndpoint
///     name: vertex_index_endpoint_deployed
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
/// variables:
///   vertexNetwork:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: network-name
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Vertex Ai Index Endpoint Deployed Index Basic Two
///
///
/// ```yaml
/// resources:
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: vertex-sa
///   basicDeployedIndex:
///     type: gcp:vertex:AiIndexEndpointDeployedIndex
///     name: basic_deployed_index
///     properties:
///       indexEndpoint: ${vertexIndexEndpointDeployed.id}
///       index: ${index.id}
///       deployedIndexId: deployed_index_id
///       reservedIpRanges:
///         - vertex-ai-range
///       enableAccessLogging: false
///       displayName: vertex-deployed-index
///       deployedIndexAuthConfig:
///         authProvider:
///           audiences:
///             - 123456-my-app
///           allowedIssuers:
///             - ${sa.email}
///       automaticResources:
///         maxReplicaCount: 4
///     options:
///       dependsOn:
///         - ${vertexIndexEndpointDeployed}
///         - ${sa}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: bucket-name
///       location: us-central1
///       uniformBucketLevelAccess: true
///   # The sample data comes from the following link:
///   # https://cloud.google.com/vertex-ai/docs/matching-engine/filtering#specify-namespaces-tokens
///   data:
///     type: gcp:storage:BucketObject
///     properties:
///       name: contents/data.json
///       bucket: ${bucket.name}
///       content: |
///         {"id": "42", "embedding": [0.5, 1.0], "restricts": [{"namespace": "class", "allow": ["cat", "pet"]},{"namespace": "category", "allow": ["feline"]}]}
///         {"id": "43", "embedding": [0.6, 1.0], "restricts": [{"namespace": "class", "allow": ["dog", "pet"]},{"namespace": "category", "allow": ["canine"]}]}
///   index:
///     type: gcp:vertex:AiIndex
///     properties:
///       labels:
///         foo: bar
///       region: us-central1
///       displayName: test-index
///       description: index for test
///       metadata:
///         contentsDeltaUri: gs://${bucket.name}/contents
///         config:
///           dimensions: 2
///           approximateNeighborsCount: 150
///           shardSize: SHARD_SIZE_SMALL
///           distanceMeasureType: DOT_PRODUCT_DISTANCE
///           algorithmConfig:
///             treeAhConfig:
///               leafNodeEmbeddingCount: 500
///               leafNodesToSearchPercent: 7
///       indexUpdateMethod: BATCH_UPDATE
///   vertexIndexEndpointDeployed:
///     type: gcp:vertex:AiIndexEndpoint
///     name: vertex_index_endpoint_deployed
///     properties:
///       displayName: sample-endpoint
///       description: A sample vertex endpoint
///       region: us-central1
///       labels:
///         label-one: value-one
///       network: projects/${project.number}/global/networks/${vertexNetwork.name}
/// variables:
///   vertexNetwork:
///     fn::invoke:
///       function: gcp:compute:getNetwork
///       arguments:
///         name: network-name
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// IndexEndpointDeployedIndex can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/indexEndpoints/{{index_endpoint}}/deployedIndex/{{deployed_index_id}}`
///
/// * `{{project}}/{{region}}/{{index_endpoint}}/{{deployed_index_id}}`
///
/// * `{{region}}/{{index_endpoint}}/{{deployed_index_id}}`
///
/// * `{{index_endpoint}}/{{deployed_index_id}}`
///
/// When using the `pulumi import` command, IndexEndpointDeployedIndex can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpointDeployedIndex:AiIndexEndpointDeployedIndex default projects/{{project}}/locations/{{region}}/indexEndpoints/{{index_endpoint}}/deployedIndex/{{deployed_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpointDeployedIndex:AiIndexEndpointDeployedIndex default {{project}}/{{region}}/{{index_endpoint}}/{{deployed_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpointDeployedIndex:AiIndexEndpointDeployedIndex default {{region}}/{{index_endpoint}}/{{deployed_index_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:vertex/aiIndexEndpointDeployedIndex:AiIndexEndpointDeployedIndex default {{index_endpoint}}/{{deployed_index_id}}
/// ```
///
pub mod ai_index_endpoint_deployed_index {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiIndexEndpointDeployedIndexArgs {
        /// A description of resources that the DeployedIndex uses, which to large degree are decided by Vertex AI, and optionally allows only a modest additional configuration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub automatic_resources: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiIndexEndpointDeployedIndexAutomaticResources,
            >,
        >,
        /// A description of resources that are dedicated to the DeployedIndex, and that need a higher degree of manual configuration. The field minReplicaCount must be set to a value strictly greater than 0, or else validation will fail. We don't provide SLA when minReplicaCount=1. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.
        /// Available machine types for SMALL shard: e2-standard-2 and all machine types available for MEDIUM and LARGE shard.
        /// Available machine types for MEDIUM shard: e2-standard-16 and all machine types available for LARGE shard.
        /// Available machine types for LARGE shard: e2-highmem-16, n2d-standard-32.
        /// n1-standard-16 and n1-standard-32 are still available, but we recommend e2-standard-16 and e2-highmem-16 for cost efficiency.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dedicated_resources: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiIndexEndpointDeployedIndexDedicatedResources,
            >,
        >,
        /// If set, the authentication is enabled for the private endpoint.
        /// Structure is documented below.
        #[builder(into, default)]
        pub deployed_index_auth_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiIndexEndpointDeployedIndexDeployedIndexAuthConfig,
            >,
        >,
        /// The user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in.
        #[builder(into)]
        pub deployed_index_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.
        /// Creating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).
        /// Note: we only support up to 5 deployment groups (not including 'default').
        #[builder(into, default)]
        pub deployment_group: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true, private endpoint's access logs are sent to Cloud Logging.
        #[builder(into, default)]
        pub enable_access_logging: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the Index this is the deployment of.
        #[builder(into)]
        pub index: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifies the index endpoint. Must be in the format
        /// 'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'
        ///
        ///
        /// - - -
        #[builder(into)]
        pub index_endpoint: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.
        /// If set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.
        /// The value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].
        /// For more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges.
        #[builder(into, default)]
        pub reserved_ip_ranges: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct AiIndexEndpointDeployedIndexResult {
        /// A description of resources that the DeployedIndex uses, which to large degree are decided by Vertex AI, and optionally allows only a modest additional configuration.
        /// Structure is documented below.
        pub automatic_resources: pulumi_wasm_rust::Output<
            super::super::types::vertex::AiIndexEndpointDeployedIndexAutomaticResources,
        >,
        /// The timestamp of when the Index was created in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// A description of resources that are dedicated to the DeployedIndex, and that need a higher degree of manual configuration. The field minReplicaCount must be set to a value strictly greater than 0, or else validation will fail. We don't provide SLA when minReplicaCount=1. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.
        /// Available machine types for SMALL shard: e2-standard-2 and all machine types available for MEDIUM and LARGE shard.
        /// Available machine types for MEDIUM shard: e2-standard-16 and all machine types available for LARGE shard.
        /// Available machine types for LARGE shard: e2-highmem-16, n2d-standard-32.
        /// n1-standard-16 and n1-standard-32 are still available, but we recommend e2-standard-16 and e2-highmem-16 for cost efficiency.
        /// Structure is documented below.
        pub dedicated_resources: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiIndexEndpointDeployedIndexDedicatedResources,
            >,
        >,
        /// If set, the authentication is enabled for the private endpoint.
        /// Structure is documented below.
        pub deployed_index_auth_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiIndexEndpointDeployedIndexDeployedIndexAuthConfig,
            >,
        >,
        /// The user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in.
        pub deployed_index_id: pulumi_wasm_rust::Output<String>,
        /// The deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.
        /// Creating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).
        /// Note: we only support up to 5 deployment groups (not including 'default').
        pub deployment_group: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, private endpoint's access logs are sent to Cloud Logging.
        pub enable_access_logging: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Index this is the deployment of.
        pub index: pulumi_wasm_rust::Output<String>,
        /// Identifies the index endpoint. Must be in the format
        /// 'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'
        ///
        ///
        /// - - -
        pub index_endpoint: pulumi_wasm_rust::Output<String>,
        /// The DeployedIndex may depend on various data on its original Index. Additionally when certain changes to the original Index are being done (e.g. when what the Index contains is being changed) the DeployedIndex may be asynchronously updated in the background to reflect these changes. If this timestamp's value is at least the [Index.update_time](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes#Index.FIELDS.update_time) of the original Index, it means that this DeployedIndex and the original Index are in sync. If this timestamp is older, then to see which updates this DeployedIndex already contains (and which it does not), one must [list](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.operations/list#google.longrunning.Operations.ListOperations) the operations that are running on the original Index. Only the successfully completed Operations with updateTime equal or before this sync time are contained in this DeployedIndex.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub index_sync_time: pulumi_wasm_rust::Output<String>,
        /// The name of the DeployedIndex resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Provides paths for users to send requests directly to the deployed index services running on Cloud via private services access. This field is populated if [network](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#IndexEndpoint.FIELDS.network) is configured.
        /// Structure is documented below.
        pub private_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::vertex::AiIndexEndpointDeployedIndexPrivateEndpoint>,
        >,
        /// A list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.
        /// If set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.
        /// The value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].
        /// For more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges.
        pub reserved_ip_ranges: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiIndexEndpointDeployedIndexArgs,
    ) -> AiIndexEndpointDeployedIndexResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let automatic_resources_binding = args
            .automatic_resources
            .get_output(context)
            .get_inner();
        let dedicated_resources_binding = args
            .dedicated_resources
            .get_output(context)
            .get_inner();
        let deployed_index_auth_config_binding = args
            .deployed_index_auth_config
            .get_output(context)
            .get_inner();
        let deployed_index_id_binding = args
            .deployed_index_id
            .get_output(context)
            .get_inner();
        let deployment_group_binding = args
            .deployment_group
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let enable_access_logging_binding = args
            .enable_access_logging
            .get_output(context)
            .get_inner();
        let index_binding = args.index.get_output(context).get_inner();
        let index_endpoint_binding = args.index_endpoint.get_output(context).get_inner();
        let reserved_ip_ranges_binding = args
            .reserved_ip_ranges
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiIndexEndpointDeployedIndex:AiIndexEndpointDeployedIndex"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "automaticResources".into(),
                    value: &automatic_resources_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedResources".into(),
                    value: &dedicated_resources_binding,
                },
                register_interface::ObjectField {
                    name: "deployedIndexAuthConfig".into(),
                    value: &deployed_index_auth_config_binding,
                },
                register_interface::ObjectField {
                    name: "deployedIndexId".into(),
                    value: &deployed_index_id_binding,
                },
                register_interface::ObjectField {
                    name: "deploymentGroup".into(),
                    value: &deployment_group_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "enableAccessLogging".into(),
                    value: &enable_access_logging_binding,
                },
                register_interface::ObjectField {
                    name: "index".into(),
                    value: &index_binding,
                },
                register_interface::ObjectField {
                    name: "indexEndpoint".into(),
                    value: &index_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "reservedIpRanges".into(),
                    value: &reserved_ip_ranges_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiIndexEndpointDeployedIndexResult {
            automatic_resources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("automaticResources"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            dedicated_resources: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dedicatedResources"),
            ),
            deployed_index_auth_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deployedIndexAuthConfig"),
            ),
            deployed_index_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deployedIndexId"),
            ),
            deployment_group: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deploymentGroup"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enable_access_logging: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enableAccessLogging"),
            ),
            index: pulumi_wasm_rust::__private::into_domain(o.extract_field("index")),
            index_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexEndpoint"),
            ),
            index_sync_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("indexSyncTime"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_endpoints: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateEndpoints"),
            ),
            reserved_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reservedIpRanges"),
            ),
        }
    }
}
