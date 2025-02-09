/// Resource for managing an AWS Service Quotas Template.
///
/// > Only the management account of an organization can alter Service Quota templates, and this must be done from the `us-east-1` region.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicequotas:Template
///     properties:
///       region: us-east-1
///       quotaCode: L-2ACBD22F
///       serviceCode: lambda
///       value: '80'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Quotas Template using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicequotas/template:Template example us-east-1,L-2ACBD22F,lambda
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateArgs {
        /// Quota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
        #[builder(into)]
        pub quota_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS Region to which the template applies.
        #[builder(into)]
        pub region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Service identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
        #[builder(into)]
        pub service_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The new, increased value for the quota.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<f64>,
    }
    #[allow(dead_code)]
    pub struct TemplateResult {
        /// Indicates whether the quota is global.
        pub global_quota: pulumi_gestalt_rust::Output<bool>,
        /// Quota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
        pub quota_code: pulumi_gestalt_rust::Output<String>,
        /// Quota name.
        pub quota_name: pulumi_gestalt_rust::Output<String>,
        /// AWS Region to which the template applies.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// Service identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
        pub service_code: pulumi_gestalt_rust::Output<String>,
        /// Service name.
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Unit of measurement.
        pub unit: pulumi_gestalt_rust::Output<String>,
        /// The new, increased value for the quota.
        pub value: pulumi_gestalt_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TemplateArgs,
    ) -> TemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let quota_code_binding = args.quota_code.get_output(context);
        let region_binding = args.region.get_output(context);
        let service_code_binding = args.service_code.get_output(context);
        let value_binding = args.value.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:servicequotas/template:Template".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quotaCode".into(),
                    value: quota_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceCode".into(),
                    value: service_code_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: value_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TemplateResult {
            global_quota: o.get_field("globalQuota"),
            quota_code: o.get_field("quotaCode"),
            quota_name: o.get_field("quotaName"),
            region: o.get_field("region"),
            service_code: o.get_field("serviceCode"),
            service_name: o.get_field("serviceName"),
            unit: o.get_field("unit"),
            value: o.get_field("value"),
        }
    }
}
