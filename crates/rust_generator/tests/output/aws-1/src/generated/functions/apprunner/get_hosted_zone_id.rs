#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_hosted_zone_id {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostedZoneIdArgs {
        /// Name of the region whose AWS App Runner service HostedZoneId is desired.
        /// Defaults to the region from the AWS provider configuration.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetHostedZoneIdResult {
        /// ID of the AWS App Runner service HostedZoneId in the selected region.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetHostedZoneIdArgs,
    ) -> GetHostedZoneIdResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:apprunner/getHostedZoneId:getHostedZoneId".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetHostedZoneIdResult {
            id: o.get_field("id"),
            region: o.get_field("region"),
        }
    }
}
