#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceAccountArgs {
        /// Name of the region whose AWS CloudTrail account ID is desired.
        /// Defaults to the region from the AWS provider configuration.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceAccountResult {
        /// ARN of the AWS CloudTrail service account in the selected region.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceAccountArgs,
    ) -> GetServiceAccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudtrail/getServiceAccount:getServiceAccount".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServiceAccountResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
        }
    }
}
