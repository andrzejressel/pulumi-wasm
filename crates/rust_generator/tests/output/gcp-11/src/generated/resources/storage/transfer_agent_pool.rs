/// Represents an On-Premises Agent pool.
///
///
/// To get more information about AgentPool, see:
///
/// * [API documentation](https://cloud.google.com/storage-transfer/docs/reference/rest/v1/projects.agentPools)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage-transfer/docs/on-prem-agent-pools)
///
/// ## Example Usage
///
/// ### Agent Pool Basic
///
///
/// ```yaml
/// resources:
///   pubsubEditorRole:
///     type: gcp:projects:IAMMember
///     name: pubsub_editor_role
///     properties:
///       project: my-project-name
///       role: roles/pubsub.editor
///       member: serviceAccount:${default.email}
///   example:
///     type: gcp:storage:TransferAgentPool
///     properties:
///       name: agent-pool-example
///       displayName: Source A to destination Z
///       bandwidthLimit:
///         limitMbps: '120'
///     options:
///       dependsOn:
///         - ${pubsubEditorRole}
/// variables:
///   default:
///     fn::invoke:
///       function: gcp:storage:getTransferProjectServiceAccount
///       arguments:
///         project: my-project-name
/// ```
///
/// ## Import
///
/// AgentPool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/agentPools/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, AgentPool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/transferAgentPool:TransferAgentPool default projects/{{project}}/agentPools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/transferAgentPool:TransferAgentPool default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/transferAgentPool:TransferAgentPool default {{name}}
/// ```
///
pub mod transfer_agent_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TransferAgentPoolArgs {
        /// Specifies the bandwidth limit details. If this field is unspecified, the default value is set as 'No Limit'.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bandwidth_limit: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::storage::TransferAgentPoolBandwidthLimit>,
        >,
        /// Specifies the client-specified AgentPool description.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the agent pool to create.
        /// The agentPoolId must meet the following requirements:
        /// * Length of 128 characters or less.
        /// * Not start with the string goog.
        /// * Start with a lowercase ASCII character, followed by:
        /// * Zero or more: lowercase Latin alphabet characters, numerals, hyphens (-), periods (.), underscores (_), or tildes (~).
        /// * One or more numerals or lowercase ASCII characters.
        /// As expressed by the regular expression: ^(?!goog)a-z?$.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TransferAgentPoolResult {
        /// Specifies the bandwidth limit details. If this field is unspecified, the default value is set as 'No Limit'.
        /// Structure is documented below.
        pub bandwidth_limit: pulumi_gestalt_rust::Output<
            Option<super::super::types::storage::TransferAgentPoolBandwidthLimit>,
        >,
        /// Specifies the client-specified AgentPool description.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the agent pool to create.
        /// The agentPoolId must meet the following requirements:
        /// * Length of 128 characters or less.
        /// * Not start with the string goog.
        /// * Start with a lowercase ASCII character, followed by:
        /// * Zero or more: lowercase Latin alphabet characters, numerals, hyphens (-), periods (.), underscores (_), or tildes (~).
        /// * One or more numerals or lowercase ASCII characters.
        /// As expressed by the regular expression: ^(?!goog)a-z?$.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Specifies the state of the AgentPool.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TransferAgentPoolArgs,
    ) -> TransferAgentPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bandwidth_limit_binding = args
            .bandwidth_limit
            .get_output(context)
            .get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/transferAgentPool:TransferAgentPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bandwidthLimit".into(),
                    value: &bandwidth_limit_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TransferAgentPoolResult {
            bandwidth_limit: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bandwidthLimit"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
