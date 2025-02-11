/// ## Example Usage
///
/// ### Network Security Intercept Deployment Group Basic
///
///
/// ```yaml
/// resources:
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: example-network
///       autoCreateSubnetworks: false
///   default:
///     type: gcp:networksecurity:InterceptDeploymentGroup
///     properties:
///       interceptDeploymentGroupId: example-dg
///       location: global
///       network: ${network.id}
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// InterceptDeploymentGroup can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/interceptDeploymentGroups/{{intercept_deployment_group_id}}`
///
/// * `{{project}}/{{location}}/{{intercept_deployment_group_id}}`
///
/// * `{{location}}/{{intercept_deployment_group_id}}`
///
/// When using the `pulumi import` command, InterceptDeploymentGroup can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeploymentGroup:InterceptDeploymentGroup default projects/{{project}}/locations/{{location}}/interceptDeploymentGroups/{{intercept_deployment_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeploymentGroup:InterceptDeploymentGroup default {{project}}/{{location}}/{{intercept_deployment_group_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/interceptDeploymentGroup:InterceptDeploymentGroup default {{location}}/{{intercept_deployment_group_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod intercept_deployment_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InterceptDeploymentGroupArgs {
        /// Required. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// intercept_deployment_group_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        #[builder(into)]
        pub intercept_deployment_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/InterceptDeploymentGroup`.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Required. Immutable. The network that is being used for the deployment. Format is:
        /// projects/{project}/global/networks/{network}.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct InterceptDeploymentGroupResult {
        /// Output only. The list of Intercept Endpoint Groups that are connected to this resource.
        /// Structure is documented below.
        pub connected_endpoint_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::networksecurity::InterceptDeploymentGroupConnectedEndpointGroup,
            >,
        >,
        /// Output only. [Output only] Create time stamp
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. Id of the requesting object
        /// If auto-generating Id server-side, remove this field and
        /// intercept_deployment_group_id from the method_signature of Create RPC
        ///
        ///
        /// - - -
        pub intercept_deployment_group_id: pulumi_gestalt_rust::Output<String>,
        /// Optional. Labels as key value pairs
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resource ID segment making up resource `name`. It identifies the resource within its parent collection as described in https://google.aip.dev/122. See documentation for resource type `networksecurity.googleapis.com/InterceptDeploymentGroup`.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Output only. A connected intercept endpoint group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Required. Immutable. The network that is being used for the deployment. Format is:
        /// projects/{project}/global/networks/{network}.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Whether reconciling is in progress, recommended per
        /// https://google.aip.dev/128.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// Output only. Current state of the deployment group.
        /// Possible values:
        /// STATE_UNSPECIFIED
        /// ACTIVE
        /// CREATING
        /// DELETING
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Output only. [Output only] Update time stamp
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InterceptDeploymentGroupArgs,
    ) -> InterceptDeploymentGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let intercept_deployment_group_id_binding = args
            .intercept_deployment_group_id
            .get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/interceptDeploymentGroup:InterceptDeploymentGroup"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "interceptDeploymentGroupId".into(),
                    value: &intercept_deployment_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        InterceptDeploymentGroupResult {
            connected_endpoint_groups: o.get_field("connectedEndpointGroups"),
            create_time: o.get_field("createTime"),
            effective_labels: o.get_field("effectiveLabels"),
            intercept_deployment_group_id: o.get_field("interceptDeploymentGroupId"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            state: o.get_field("state"),
            update_time: o.get_field("updateTime"),
        }
    }
}
