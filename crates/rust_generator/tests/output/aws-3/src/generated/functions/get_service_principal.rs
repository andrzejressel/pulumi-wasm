#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_principal {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServicePrincipalArgs {
        /// Region you'd like the SPN for. By default, uses the current region.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the service you want to generate a Service Principal Name for.
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServicePrincipalResult {
        /// Identifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Service Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Region identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
        pub region: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        /// Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
        pub suffix: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServicePrincipalArgs,
    ) -> GetServicePrincipalResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let region_binding = args.region.get_output(context);
        let service_name_binding = args.service_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getServicePrincipal:getServicePrincipal".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceName".into(),
                    value: service_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServicePrincipalResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            region: o.get_field("region"),
            service_name: o.get_field("serviceName"),
            suffix: o.get_field("suffix"),
        }
    }
}
