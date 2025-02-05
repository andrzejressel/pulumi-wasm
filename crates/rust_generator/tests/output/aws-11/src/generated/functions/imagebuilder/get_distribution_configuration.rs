pub mod get_distribution_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDistributionConfigurationArgs {
        /// ARN of the distribution configuration.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the distribution configuration.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDistributionConfigurationResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date the distribution configuration was created.
        pub date_created: pulumi_wasm_rust::Output<String>,
        /// Date the distribution configuration was updated.
        pub date_updated: pulumi_wasm_rust::Output<String>,
        /// Description of the container distribution configuration.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Set of distributions.
        pub distributions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::imagebuilder::GetDistributionConfigurationDistribution,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the distribution configuration.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the distribution configuration.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDistributionConfigurationArgs,
    ) -> GetDistributionConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:imagebuilder/getDistributionConfiguration:getDistributionConfiguration"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDistributionConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            date_created: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            date_updated: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dateUpdated"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            distributions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("distributions"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
