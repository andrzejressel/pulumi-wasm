pub mod get_delegation_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDelegationSetArgs {
        /// Delegation set ID.
        ///
        /// The following attribute is additionally exported:
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDelegationSetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub caller_reference: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDelegationSetArgs,
    ) -> GetDelegationSetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:route53/getDelegationSet:getDelegationSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDelegationSetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            caller_reference: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("callerReference"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name_servers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nameServers"),
            ),
        }
    }
}
