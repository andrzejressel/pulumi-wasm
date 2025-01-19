/// Resource for managing an AWS Service Quotas Template.
///
/// > Only the management account of an organization can alter Service Quota templates, and this must be done from the `us-east-1` region.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:servicequotas:Template
///     properties:
///       region: us-east-1
///       quotaCode: L-2ACBD22F
///       serviceCode: lambda
///       value: '80'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Service Quotas Template using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:servicequotas/template:Template example us-east-1,L-2ACBD22F,lambda
/// ```
pub mod template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateArgs {
        /// Quota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
        #[builder(into)]
        pub quota_code: pulumi_wasm_rust::Output<String>,
        /// AWS Region to which the template applies.
        #[builder(into)]
        pub region: pulumi_wasm_rust::Output<String>,
        /// Service identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
        #[builder(into)]
        pub service_code: pulumi_wasm_rust::Output<String>,
        /// The new, increased value for the quota.
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<f64>,
    }
    #[allow(dead_code)]
    pub struct TemplateResult {
        /// Indicates whether the quota is global.
        pub global_quota: pulumi_wasm_rust::Output<bool>,
        /// Quota identifier. To find the quota code for a specific quota, use the aws.servicequotas.ServiceQuota data source.
        pub quota_code: pulumi_wasm_rust::Output<String>,
        /// Quota name.
        pub quota_name: pulumi_wasm_rust::Output<String>,
        /// AWS Region to which the template applies.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Service identifier. To find the service code value for an AWS service, use the aws.servicequotas.getService data source.
        pub service_code: pulumi_wasm_rust::Output<String>,
        /// Service name.
        pub service_name: pulumi_wasm_rust::Output<String>,
        /// Unit of measurement.
        pub unit: pulumi_wasm_rust::Output<String>,
        /// The new, increased value for the quota.
        pub value: pulumi_wasm_rust::Output<f64>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TemplateArgs) -> TemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let quota_code_binding = args.quota_code.get_inner();
        let region_binding = args.region.get_inner();
        let service_code_binding = args.service_code.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicequotas/template:Template".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "quotaCode".into(),
                    value: &quota_code_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "globalQuota".into(),
                },
                register_interface::ResultField {
                    name: "quotaCode".into(),
                },
                register_interface::ResultField {
                    name: "quotaName".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "serviceCode".into(),
                },
                register_interface::ResultField {
                    name: "serviceName".into(),
                },
                register_interface::ResultField {
                    name: "unit".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TemplateResult {
            global_quota: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalQuota").unwrap(),
            ),
            quota_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaCode").unwrap(),
            ),
            quota_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("quotaName").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            service_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceCode").unwrap(),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceName").unwrap(),
            ),
            unit: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("unit").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
