pub mod get_received_license {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReceivedLicenseArgs {
        /// The ARN of the received license you want data for.
        #[builder(into)]
        pub license_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReceivedLicenseResult {
        /// Granted license beneficiary. This is in the form of the ARN of the root user of the account.
        pub beneficiary: pulumi_wasm_rust::Output<String>,
        /// Configuration for consumption of the license. Detailed below
        pub consumption_configurations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseConsumptionConfiguration,
            >,
        >,
        /// Creation time of the granted license in RFC 3339 format.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// License entitlements. Detailed below
        pub entitlements: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseEntitlement,
            >,
        >,
        /// Home Region of the granted license.
        pub home_region: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Granted license issuer. Detailed below
        pub issuers: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::licensemanager::GetReceivedLicenseIssuer>,
        >,
        /// Amazon Resource Name (ARN) of the license.
        pub license_arn: pulumi_wasm_rust::Output<String>,
        /// Granted license metadata. This is in the form of a set of all meta data. Detailed below
        pub license_metadatas: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseLicenseMetadata,
            >,
        >,
        /// License name.
        pub license_name: pulumi_wasm_rust::Output<String>,
        /// Product name.
        /// * `product_sku ` - Product SKU.
        pub product_name: pulumi_wasm_rust::Output<String>,
        pub product_sku: pulumi_wasm_rust::Output<String>,
        /// Granted license received metadata. Detailed below
        pub received_metadatas: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseReceivedMetadata,
            >,
        >,
        /// Granted license status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Date and time range during which the granted license is valid, in ISO8601-UTC format. Detailed below
        pub validities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::licensemanager::GetReceivedLicenseValidity>,
        >,
        /// Version of the granted license.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetReceivedLicenseArgs,
    ) -> GetReceivedLicenseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let license_arn_binding = args.license_arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:licensemanager/getReceivedLicense:getReceivedLicense".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "licenseArn".into(),
                    value: &license_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "beneficiary".into(),
                },
                register_interface::ResultField {
                    name: "consumptionConfigurations".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "entitlements".into(),
                },
                register_interface::ResultField {
                    name: "homeRegion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "issuers".into(),
                },
                register_interface::ResultField {
                    name: "licenseArn".into(),
                },
                register_interface::ResultField {
                    name: "licenseMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "licenseName".into(),
                },
                register_interface::ResultField {
                    name: "productName".into(),
                },
                register_interface::ResultField {
                    name: "productSku".into(),
                },
                register_interface::ResultField {
                    name: "receivedMetadatas".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "validities".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReceivedLicenseResult {
            beneficiary: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("beneficiary").unwrap(),
            ),
            consumption_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("consumptionConfigurations").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            entitlements: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entitlements").unwrap(),
            ),
            home_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("homeRegion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            issuers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("issuers").unwrap(),
            ),
            license_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseArn").unwrap(),
            ),
            license_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseMetadatas").unwrap(),
            ),
            license_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseName").unwrap(),
            ),
            product_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productName").unwrap(),
            ),
            product_sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productSku").unwrap(),
            ),
            received_metadatas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("receivedMetadatas").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            validities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validities").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
