pub mod get_session_context {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSessionContextArgs {
        /// ARN for an assumed role.
        ///
        /// > If `arn` is a non-role ARN, Pulumi gives no error and `issuer_arn` will be equal to the `arn` value. For STS assumed-role ARNs, Pulumi gives an error if the identified IAM role does not exist.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSessionContextResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IAM source role ARN if `arn` corresponds to an STS assumed role. Otherwise, `issuer_arn` is equal to `arn`.
        pub issuer_arn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the IAM role that issues the STS assumed role.
        pub issuer_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the source role. Only available if `arn` corresponds to an STS assumed role.
        pub issuer_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the STS session. Only available if `arn` corresponds to an STS assumed role.
        pub session_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSessionContextArgs,
    ) -> GetSessionContextResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getSessionContext:getSessionContext".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSessionContextResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            issuer_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuerArn"),
            ),
            issuer_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuerId"),
            ),
            issuer_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuerName"),
            ),
            session_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionName"),
            ),
        }
    }
}
