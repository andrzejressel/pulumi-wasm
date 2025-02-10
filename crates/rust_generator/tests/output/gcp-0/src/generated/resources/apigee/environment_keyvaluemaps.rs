/// Collection of key/value string pairs.
///
///
/// To get more information about EnvironmentKeyvaluemaps, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keyvaluemaps/create)
/// * How-to Guides
///     * [Using key value maps](https://cloud.google.com/apigee/docs/api-platform/cache/key-value-maps)
///
/// ## Example Usage
///
/// ### Apigee Environment Keyvaluemaps Basic
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
///       envId: ${apigeeEnvironment.id}
///       name: tf-test-env-kvms
///     options:
///       dependsOn:
///         - ${apigeeOrg}
///         - ${apigeeEnvironment}
///         - ${apigeeInstance}
///         - ${apigeeInstanceAttachment}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// EnvironmentKeyvaluemaps can be imported using any of these accepted formats:
///
/// * `{{env_id}}/keyvaluemaps/{{name}}`
///
/// * `{{env_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvironmentKeyvaluemaps can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/environmentKeyvaluemaps:EnvironmentKeyvaluemaps default {{env_id}}/keyvaluemaps/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/environmentKeyvaluemaps:EnvironmentKeyvaluemaps default {{env_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod environment_keyvaluemaps {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentKeyvaluemapsArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub env_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. ID of the key value map.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EnvironmentKeyvaluemapsResult {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/environments/{{env_name}}`.
        ///
        ///
        /// - - -
        pub env_id: pulumi_gestalt_rust::Output<String>,
        /// Required. ID of the key value map.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvironmentKeyvaluemapsArgs,
    ) -> EnvironmentKeyvaluemapsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let env_id_binding = args.env_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/environmentKeyvaluemaps:EnvironmentKeyvaluemaps".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "envId".into(),
                    value: env_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvironmentKeyvaluemapsResult {
            env_id: o.get_field("envId"),
            name: o.get_field("name"),
        }
    }
}
