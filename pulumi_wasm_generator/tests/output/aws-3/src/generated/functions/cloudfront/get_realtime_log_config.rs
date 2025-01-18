pub mod get_realtime_log_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRealtimeLogConfigArgs {
        /// Unique name to identify this real-time log configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetRealtimeLogConfigResult {
        /// ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// (Required) Amazon Kinesis data streams where real-time log data is sent.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cloudfront::GetRealtimeLogConfigEndpoint>,
        >,
        /// (Required) Fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        pub fields: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// (Required) Sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        pub sampling_rate: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRealtimeLogConfigArgs) -> GetRealtimeLogConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getRealtimeLogConfig:getRealtimeLogConfig".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "fields".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "samplingRate".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRealtimeLogConfigResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fields").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sampling_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samplingRate").unwrap(),
            ),
        }
    }
}
