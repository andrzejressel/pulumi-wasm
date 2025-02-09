/// Creates key value entries in a key value map scoped to an environment.
///
///
/// To get more information about EnvironmentKeyvaluemapsEntries, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.keyvaluemaps.entries/create)
/// * How-to Guides
///     * [Using key value maps](https://cloud.google.com/apigee/docs/api-platform/cache/key-value-maps)
///
/// ## Example Usage
///
/// ### Apigee Environment Keyvaluemaps Entries Basic
///
///
/// ```yaml
/// resources:
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${current.project}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///   apigeeEnvironment:
///     type: gcp:apigee:Environment
///     name: apigee_environment
///     properties:
///       orgId: ${apigeeOrg.id}
///       name: tf-test-env
///       description: Apigee Environment
///       displayName: Apigee Environment
///   apigeeInstance:
///     type: gcp:apigee:Instance
///     name: apigee_instance
///     properties:
///       name: tf-test-instance
///       location: us-central1
///       orgId: ${apigeeOrg.id}
///   apigeeInstanceAttachment:
///     type: gcp:apigee:InstanceAttachment
///     name: apigee_instance_attachment
///     properties:
///       instanceId: ${apigeeInstance.id}
///       environment: ${apigeeEnvironment.name}
///   apigeeEnvironmentKeyvaluemaps:
///     type: gcp:apigee:EnvironmentKeyvaluemaps
///     name: apigee_environment_keyvaluemaps
///     properties:
///       envId: ${createApigeeEnvironment.id}
///       name: tf-test-env-kvms
///     options:
///       dependsOn:
///         - ${apigeeOrg}
///         - ${apigeeEnvironment}
///         - ${apigeeInstance}
///         - ${apigeeInstanceAttachment}
///   apigeeEnvironmentKeyvaluemapsEntries:
///     type: gcp:apigee:EnvironmentKeyvaluemapsEntries
///     name: apigee_environment_keyvaluemaps_entries
///     properties:
///       envKeyvaluemapId: ${apigeeEnvironmentKeyvaluemaps.id}
///       name: testName
///       value: testValue
///     options:
///       dependsOn:
///         - ${apigeeOrg}
///         - ${apigeeEnvironment}
///         - ${apigeeInstance}
///         - ${apigeeInstanceAttachment}
///         - ${apigeeEnvironmentKeyvaluemaps}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// EnvironmentKeyvaluemapsEntries can be imported using any of these accepted formats:
///
/// * `{{env_keyvaluemap_id}}/entries/{{name}}`
///
/// * `{{env_keyvaluemap_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvironmentKeyvaluemapsEntries can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/environmentKeyvaluemapsEntries:EnvironmentKeyvaluemapsEntries default {{env_keyvaluemap_id}}/entries/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/environmentKeyvaluemapsEntries:EnvironmentKeyvaluemapsEntries default {{env_keyvaluemap_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_keyvaluemaps_entries {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentKeyvaluemapsEntriesArgs {
        /// The Apigee environment keyvalumaps Id associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}/keyvaluemaps/{{keyvaluemap_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_keyvaluemap_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Resource URI that can be used to identify the scope of the key value map entries.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Data or payload that is being retrieved and associated with the unique key.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentKeyvaluemapsEntriesResult {
        /// The Apigee environment keyvalumaps Id associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}/keyvaluemaps/{{keyvaluemap_name}}`.
        ///
        ///
        /// - - -
        pub env_keyvaluemap_id: pulumi_gestalt_rust::Output<String>,
        /// Required. Resource URI that can be used to identify the scope of the key value map entries.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. Data or payload that is being retrieved and associated with the unique key.
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentKeyvaluemapsEntriesArgs,
    ) -> EnvironmentKeyvaluemapsEntriesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let env_keyvaluemap_id_binding = args.env_keyvaluemap_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/environmentKeyvaluemapsEntries:EnvironmentKeyvaluemapsEntries"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envKeyvaluemapId".into(),
                    value: env_keyvaluemap_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentKeyvaluemapsEntriesResult {
            env_keyvaluemap_id: o.get_field("envKeyvaluemapId"),
            name: o.get_field("name"),
            value: o.get_field("value"),
        }
    }
}
