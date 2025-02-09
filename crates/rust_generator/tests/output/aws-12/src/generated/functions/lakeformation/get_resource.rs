#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resource {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourceArgs {
        /// ARN of the resource, an S3 path.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetResourceResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Date and time the resource was last modified in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
        pub last_modified: pulumi_gestalt_rust::Output<String>,
        /// Role that the resource was registered with.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResourceArgs,
    ) -> GetResourceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding_1 = args.arn.get_output(context);
        let arn_binding = arn_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:lakeformation/getResource:getResource".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResourceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            last_modified: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModified"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
        }
    }
}
