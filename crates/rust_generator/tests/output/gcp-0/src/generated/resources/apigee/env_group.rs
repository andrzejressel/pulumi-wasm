/// An `Environment group` in Apigee.
///
///
/// To get more information about Envgroup, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.envgroups/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ### Apigee Environment Group Basic
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
///   envGrp:
///     type: gcp:apigee:EnvGroup
///     name: env_grp
///     properties:
///       name: my-envgroup
///       hostnames:
///         - abc.foo.com
///       orgId: ${apigeeOrg.id}
/// variables:
///   current:
///     fn::invoke:
///       function: gcp:organizations:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Envgroup can be imported using any of these accepted formats:
///
/// * `{{org_id}}/envgroups/{{name}}`
///
/// * `{{org_id}}/{{name}}`
///
/// When using the `pulumi import` command, Envgroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroup:EnvGroup default {{org_id}}/envgroups/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroup:EnvGroup default {{org_id}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod env_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvGroupArgs {
        /// Hostnames of the environment group.
        #[builder(into, default)]
        pub hostnames: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The resource ID of the environment group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Apigee Organization associated with the Apigee environment group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvGroupResult {
        /// Hostnames of the environment group.
        pub hostnames: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The resource ID of the environment group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Apigee Organization associated with the Apigee environment group,
        /// in the format `organizations/{{org_name}}`.
        ///
        ///
        /// - - -
        pub org_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnvGroupArgs,
    ) -> EnvGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hostnames_binding = args.hostnames.get_output(context);
        let name_binding = args.name.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigee/envGroup:EnvGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostnames".into(),
                    value: hostnames_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: org_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnvGroupResult {
            hostnames: o.get_field("hostnames"),
            name: o.get_field("name"),
            org_id: o.get_field("orgId"),
        }
    }
}
