pub mod get_application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApplicationArgs {
        /// ARN of the application.
        #[builder(into)]
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// Options for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
        #[builder(into, default)]
        pub portal_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ssoadmin::GetApplicationPortalOption>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetApplicationResult {
        /// AWS account ID.
        pub application_account: pulumi_wasm_rust::Output<String>,
        pub application_arn: pulumi_wasm_rust::Output<String>,
        /// ARN of the application provider.
        pub application_provider_arn: pulumi_wasm_rust::Output<String>,
        /// Description of the application.
        pub description: pulumi_wasm_rust::Output<String>,
        /// ARN of the application.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the application.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Options for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
        pub portal_options: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ssoadmin::GetApplicationPortalOption>>,
        >,
        /// Status of the application.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApplicationArgs) -> GetApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_arn_binding = args.application_arn.get_inner();
        let portal_options_binding = args.portal_options.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssoadmin/getApplication:getApplication".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationArn".into(),
                    value: &application_arn_binding,
                },
                register_interface::ObjectField {
                    name: "portalOptions".into(),
                    value: &portal_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationAccount".into(),
                },
                register_interface::ResultField {
                    name: "applicationArn".into(),
                },
                register_interface::ResultField {
                    name: "applicationProviderArn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "portalOptions".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApplicationResult {
            application_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationAccount").unwrap(),
            ),
            application_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationArn").unwrap(),
            ),
            application_provider_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationProviderArn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            portal_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("portalOptions").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
