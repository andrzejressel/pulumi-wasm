pub mod get_report_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReportPlanArgs {
        /// Backup report plan name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Metadata that you can assign to help organize the report plans you create.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetReportPlanResult {
        /// ARN of the backup report plan.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Date and time that a report plan is created, in Unix format and Coordinated Universal Time (UTC).
        pub creation_time: pulumi_wasm_rust::Output<String>,
        /// Deployment status of a report plan. The statuses are: `CREATE_IN_PROGRESS` | `UPDATE_IN_PROGRESS` | `DELETE_IN_PROGRESS` | `COMPLETED`.
        pub deployment_status: pulumi_wasm_rust::Output<String>,
        /// Description of the report plan.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// An object that contains information about where and how to deliver your reports, specifically your Amazon S3 bucket name, S3 key prefix, and the formats of your reports. Detailed below.
        pub report_delivery_channels: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::backup::GetReportPlanReportDeliveryChannel>,
        >,
        /// An object that identifies the report template for the report. Reports are built using a report template. Detailed below.
        pub report_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::backup::GetReportPlanReportSetting>,
        >,
        /// Metadata that you can assign to help organize the report plans you create.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetReportPlanArgs) -> GetReportPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:backup/getReportPlan:getReportPlan".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "deploymentStatus".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "reportDeliveryChannels".into(),
                },
                register_interface::ResultField {
                    name: "reportSettings".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReportPlanResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            deployment_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deploymentStatus").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            report_delivery_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportDeliveryChannels").unwrap(),
            ),
            report_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reportSettings").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
