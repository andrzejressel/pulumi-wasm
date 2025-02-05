pub mod get_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// Name of the configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationResult {
        /// ARN of the configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of Apache Kafka versions which can use this configuration.
        pub kafka_versions: pulumi_wasm_rust::Output<Vec<String>>,
        /// Latest revision of the configuration.
        pub latest_revision: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contents of the server.properties file.
        pub server_properties: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConfigurationArgs,
    ) -> GetConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:msk/getConfiguration:getConfiguration".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfigurationResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kafka_versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kafkaVersions"),
            ),
            latest_revision: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestRevision"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            server_properties: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverProperties"),
            ),
        }
    }
}
