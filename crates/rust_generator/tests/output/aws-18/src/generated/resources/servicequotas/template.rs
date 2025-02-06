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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TemplateArgs,
    ) -> TemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let quota_code_binding = args.quota_code.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let service_code_binding = args.service_code.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicequotas/template:Template".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "quotaCode".into(),
                    value: &quota_code_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TemplateResult {
            global_quota: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("globalQuota"),
            ),
            quota_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("quotaCode"),
            ),
            quota_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("quotaName"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            service_code: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceCode"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            unit: pulumi_gestalt_rust::__private::into_domain(o.extract_field("unit")),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
