/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:detective:Graph
///     properties:
///       enable: true
///   exampleOrganizationConfiguration:
///     type: aws:detective:OrganizationConfiguration
///     name: example
///     properties:
///       autoEnable: true
///       graphArn: ${example.graphArn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_detective_organization_admin_account` using the behavior graph ARN. For example:
///
/// ```sh
/// $ pulumi import aws:detective/organizationConfiguration:OrganizationConfiguration example arn:aws:detective:us-east-1:123456789012:graph:00b00fd5aecc0ab60a708659477e9617
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationConfigurationArgs {
        /// When this setting is enabled, all new accounts that are created in, or added to, the organization are added as a member accounts of the organization’s Detective delegated administrator and Detective is enabled in that AWS Region.
        #[builder(into)]
        pub auto_enable: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// ARN of the behavior graph.
        #[builder(into)]
        pub graph_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationConfigurationResult {
        /// When this setting is enabled, all new accounts that are created in, or added to, the organization are added as a member accounts of the organization’s Detective delegated administrator and Detective is enabled in that AWS Region.
        pub auto_enable: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the behavior graph.
        pub graph_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationConfigurationArgs,
    ) -> OrganizationConfigurationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_enable_binding = args.auto_enable.get_output(context);
        let graph_arn_binding = args.graph_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:detective/organizationConfiguration:OrganizationConfiguration"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoEnable".into(),
                    value: auto_enable_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "graphArn".into(),
                    value: graph_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationConfigurationResult {
            auto_enable: o.get_field("autoEnable"),
            graph_arn: o.get_field("graphArn"),
        }
    }
}
