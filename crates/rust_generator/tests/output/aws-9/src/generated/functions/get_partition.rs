pub mod get_partition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPartitionArgs {
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPartitionResult {
        /// Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
        pub dns_suffix: pulumi_wasm_rust::Output<String>,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        pub id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
        pub partition: pulumi_wasm_rust::Output<String>,
        /// Prefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
        pub reverse_dns_prefix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPartitionArgs,
    ) -> GetPartitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getPartition:getPartition".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPartitionResult {
            dns_suffix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSuffix"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            partition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("partition"),
            ),
            reverse_dns_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reverseDnsPrefix"),
            ),
        }
    }
}
