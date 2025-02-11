/// Manages an Amazon Managed Service for Prometheus (AMP) Alert Manager Definition
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let demo = workspace::create("demo", WorkspaceArgs::builder().build_struct());
///     let demoAlertManagerDefinition = alert_manager_definition::create(
///         "demoAlertManagerDefinition",
///         AlertManagerDefinitionArgs::builder()
///             .definition(
///                 "alertmanager_config: |\n  route:\n    receiver: 'default'\n  receivers:\n    - name: 'default'",
///             )
///             .workspace_id("${demo.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the prometheus alert manager definition using the workspace identifier. For example:
///
/// ```sh
/// $ pulumi import aws:amp/alertManagerDefinition:AlertManagerDefinition demo ws-C6DCB907-F2D7-4D96-957B-66691F865D8B
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod alert_manager_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AlertManagerDefinitionArgs {
        /// the alert manager definition that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-alert-manager.html).
        #[builder(into)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the prometheus workspace the alert manager definition should be linked to
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AlertManagerDefinitionResult {
        /// the alert manager definition that you want to be applied. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-alert-manager.html).
        pub definition: pulumi_gestalt_rust::Output<String>,
        /// ID of the prometheus workspace the alert manager definition should be linked to
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AlertManagerDefinitionArgs,
    ) -> AlertManagerDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let definition_binding = args.definition.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amp/alertManagerDefinition:AlertManagerDefinition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "definition".into(),
                    value: &definition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AlertManagerDefinitionResult {
            definition: o.get_field("definition"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}
