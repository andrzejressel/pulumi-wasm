/// Allocates a static IP address.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones in Amazon Lightsail"](https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail) for more details
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = static_ip::create(
///         "test",
///         StaticIpArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
pub mod static_ip {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StaticIpArgs {
        /// The name for the allocated static IP
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StaticIpResult {
        /// The ARN of the Lightsail static IP
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The allocated static IP address
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// The name for the allocated static IP
        pub name: pulumi_wasm_rust::Output<String>,
        /// The support code.
        pub support_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: StaticIpArgs) -> StaticIpResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/staticIp:StaticIp".into(),
            name: name.to_string(),
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
                    name: "ipAddress".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StaticIpResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddress").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
            ),
        }
    }
}
