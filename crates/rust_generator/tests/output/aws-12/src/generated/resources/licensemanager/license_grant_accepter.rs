/// Accepts a License Manager grant. This allows for sharing licenses with other aws accounts.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = license_grant_accepter::create(
///         "test",
///         LicenseGrantAccepterArgs::builder()
///             .grant_arn(
///                 "arn:aws:license-manager::123456789012:grant:g-1cf9fba4ba2f42dcab11c686c4b4d329",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_licensemanager_grant_accepter` using the grant arn. For example:
///
/// ```sh
/// $ pulumi import aws:licensemanager/licenseGrantAccepter:LicenseGrantAccepter test arn:aws:license-manager::123456789012:grant:g-1cf9fba4ba2f42dcab11c686c4b4d329
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod license_grant_accepter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LicenseGrantAccepterArgs {
        /// The ARN of the grant to accept.
        #[builder(into)]
        pub grant_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LicenseGrantAccepterResult {
        /// A list of the allowed operations for the grant.
        pub allowed_operations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ARN of the grant to accept.
        pub grant_arn: pulumi_gestalt_rust::Output<String>,
        /// The home region for the license.
        pub home_region: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the license for the grant.
        pub license_arn: pulumi_gestalt_rust::Output<String>,
        /// The Name of the grant.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent ARN.
        pub parent_arn: pulumi_gestalt_rust::Output<String>,
        /// The target account for the grant.
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
        args: LicenseGrantAccepterArgs,
    ) -> LicenseGrantAccepterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let grant_arn_binding = args.grant_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:licensemanager/licenseGrantAccepter:LicenseGrantAccepter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grantArn".into(),
                    value: grant_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LicenseGrantAccepterResult {
            allowed_operations: o.get_field("allowedOperations"),
            grant_arn: o.get_field("grantArn"),
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
