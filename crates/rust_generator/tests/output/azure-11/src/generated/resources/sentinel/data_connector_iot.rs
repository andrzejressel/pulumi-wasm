/// Manages an Iot Data Connector.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAnalyticsWorkspace = analytics_workspace::create(
///         "exampleAnalyticsWorkspace",
///         AnalyticsWorkspaceArgs::builder()
///             .location("${example.location}")
///             .name("example-workspace")
///             .resource_group_name("${example.name}")
///             .sku("PerGB2018")
///             .build_struct(),
///     );
///     let exampleDataConnectorIot = data_connector_iot::create(
///         "exampleDataConnectorIot",
///         DataConnectorIotArgs::builder()
///             .log_analytics_workspace_id(
///                 "${exampleLogAnalyticsWorkspaceOnboarding.workspaceId}",
///             )
///             .name("example")
///             .build_struct(),
///     );
///     let exampleLogAnalyticsWorkspaceOnboarding = log_analytics_workspace_onboarding::create(
///         "exampleLogAnalyticsWorkspaceOnboarding",
///         LogAnalyticsWorkspaceOnboardingArgs::builder()
///             .workspace_id("${exampleAnalyticsWorkspace.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Iot Data Connectors can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:sentinel/dataConnectorIot:DataConnectorIot example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.OperationalInsights/workspaces/workspace1/providers/Microsoft.SecurityInsights/dataConnectors/dc1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_connector_iot {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataConnectorIotArgs {
        /// The ID of the Log Analytics Workspace that this Iot Data Connector resides in. Changing this forces a new Iot Data Connector to be created.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Iot Data Connector. Changing this forces a new Iot Data Connector to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the subscription that this Iot Data Connector connects to. Changing this forces a new Iot Data Connector to be created.
        #[builder(into, default)]
        pub subscription_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataConnectorIotResult {
        /// The ID of the Log Analytics Workspace that this Iot Data Connector resides in. Changing this forces a new Iot Data Connector to be created.
        pub log_analytics_workspace_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Iot Data Connector. Changing this forces a new Iot Data Connector to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subscription that this Iot Data Connector connects to. Changing this forces a new Iot Data Connector to be created.
        pub subscription_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DataConnectorIotArgs,
    ) -> DataConnectorIotResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let log_analytics_workspace_id_binding_1 = args
            .log_analytics_workspace_id
            .get_output(context);
        let log_analytics_workspace_id_binding = log_analytics_workspace_id_binding_1
            .get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let subscription_id_binding_1 = args.subscription_id.get_output(context);
        let subscription_id_binding = subscription_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:sentinel/dataConnectorIot:DataConnectorIot".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subscriptionId".into(),
                    value: &subscription_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataConnectorIotResult {
            log_analytics_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logAnalyticsWorkspaceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subscription_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subscriptionId"),
            ),
        }
    }
}
