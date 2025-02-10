/// Provides a License Manager grant. This allows for sharing licenses with other AWS accounts.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:licensemanager:LicenseGrant
///     properties:
///       name: share-license-with-account
///       allowedOperations:
///         - ListPurchasedLicenses
///         - CheckoutLicense
///         - CheckInLicense
///         - ExtendConsumptionLicense
///         - CreateToken
///       licenseArn: arn:aws:license-manager::111111111111:license:l-exampleARN
///       principal: arn:aws:iam::111111111112:root
///       homeRegion: us-east-1
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_licensemanager_grant` using the grant arn. For example:
///
/// ```sh
/// $ pulumi import aws:licensemanager/licenseGrant:LicenseGrant test arn:aws:license-manager::123456789011:grant:g-01d313393d9e443d8664cc054db1e089
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod license_grant {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseGrantArgs {
        /// A list of the allowed operations for the grant. This is a subset of the allowed operations on the license.
        #[builder(into)]
        pub allowed_operations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ARN of the license to grant.
        #[builder(into)]
        pub license_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the grant.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The target account for the grant in the form of the ARN for an account principal of the root user.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LicenseGrantResult {
        /// A list of the allowed operations for the grant. This is a subset of the allowed operations on the license.
        pub allowed_operations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The grant ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The home region for the license.
        pub home_region: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the license to grant.
        pub license_arn: pulumi_gestalt_rust::Output<String>,
        /// The Name of the grant.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent ARN.
        pub parent_arn: pulumi_gestalt_rust::Output<String>,
        /// The target account for the grant in the form of the ARN for an account principal of the root user.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The grant status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The grant version.
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LicenseGrantArgs,
    ) -> LicenseGrantResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowed_operations_binding = args.allowed_operations.get_output(context);
        let license_arn_binding = args.license_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_binding = args.principal.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:licensemanager/licenseGrant:LicenseGrant".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowedOperations".into(),
                    value: allowed_operations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseArn".into(),
                    value: license_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principal".into(),
                    value: principal_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LicenseGrantResult {
            allowed_operations: o.get_field("allowedOperations"),
            arn: o.get_field("arn"),
            home_region: o.get_field("homeRegion"),
            license_arn: o.get_field("licenseArn"),
            name: o.get_field("name"),
            parent_arn: o.get_field("parentArn"),
            principal: o.get_field("principal"),
            status: o.get_field("status"),
            version: o.get_field("version"),
        }
    }
}
