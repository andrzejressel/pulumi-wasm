pub mod get_received_license {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReceivedLicenseArgs {
        /// The ARN of the received license you want data for.
        #[builder(into)]
        pub license_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReceivedLicenseResult {
        /// Granted license beneficiary. This is in the form of the ARN of the root user of the account.
        pub beneficiary: pulumi_gestalt_rust::Output<String>,
        /// Configuration for consumption of the license. Detailed below
        pub consumption_configurations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseConsumptionConfiguration,
            >,
        >,
        /// Creation time of the granted license in RFC 3339 format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// License entitlements. Detailed below
        pub entitlements: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseEntitlement,
            >,
        >,
        /// Home Region of the granted license.
        pub home_region: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Granted license issuer. Detailed below
        pub issuers: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::licensemanager::GetReceivedLicenseIssuer>,
        >,
        /// Amazon Resource Name (ARN) of the license.
        pub license_arn: pulumi_gestalt_rust::Output<String>,
        /// Granted license metadata. This is in the form of a set of all meta data. Detailed below
        pub license_metadatas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseLicenseMetadata,
            >,
        >,
        /// License name.
        pub license_name: pulumi_gestalt_rust::Output<String>,
        /// Product name.
        /// * `product_sku ` - Product SKU.
        pub product_name: pulumi_gestalt_rust::Output<String>,
        pub product_sku: pulumi_gestalt_rust::Output<String>,
        /// Granted license received metadata. Detailed below
        pub received_metadatas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::licensemanager::GetReceivedLicenseReceivedMetadata,
            >,
        >,
        /// Granted license status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Date and time range during which the granted license is valid, in ISO8601-UTC format. Detailed below
        pub validities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::licensemanager::GetReceivedLicenseValidity>,
        >,
        /// Version of the granted license.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReceivedLicenseArgs,
    ) -> GetReceivedLicenseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReceivedLicenseResult {
            beneficiary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("beneficiary"),
            ),
            consumption_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("consumptionConfigurations"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            entitlements: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entitlements"),
            ),
            home_region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("homeRegion"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            issuers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuers"),
            ),
            license_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseArn"),
            ),
            license_metadatas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseMetadatas"),
            ),
            license_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseName"),
            ),
            product_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productName"),
            ),
            product_sku: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productSku"),
            ),
            received_metadatas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("receivedMetadatas"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            validities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validities"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
