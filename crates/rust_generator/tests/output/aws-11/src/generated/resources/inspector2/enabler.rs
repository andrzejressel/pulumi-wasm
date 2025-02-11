/// Resource for enabling Amazon Inspector resource scans.
///
/// This resource must be created in the Organization's Administrator Account.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = enabler::create(
///         "example",
///         EnablerArgs::builder()
///             .account_ids(vec!["123456789012",])
///             .resource_types(vec!["EC2",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### For the Calling Account
///
/// ```yaml
/// resources:
///   test:
///     type: aws:inspector2:Enabler
///     properties:
///       accountIds:
///         - ${current.accountId}
///       resourceTypes:
///         - ECR
///         - EC2
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enabler {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnablerArgs {
        /// Set of account IDs.
        /// Can contain one of: the Organization's Administrator Account, or one or more Member Accounts.
        #[builder(into)]
        pub account_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Type of resources to scan.
        /// Valid values are `EC2`, `ECR`, `LAMBDA` and `LAMBDA_CODE`.
        /// At least one item is required.
        #[builder(into)]
        pub resource_types: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct EnablerResult {
        /// Set of account IDs.
        /// Can contain one of: the Organization's Administrator Account, or one or more Member Accounts.
        pub account_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Type of resources to scan.
        /// Valid values are `EC2`, `ECR`, `LAMBDA` and `LAMBDA_CODE`.
        /// At least one item is required.
        pub resource_types: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EnablerArgs,
    ) -> EnablerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_ids_binding = args.account_ids.get_output(context);
        let resource_types_binding = args.resource_types.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:inspector2/enabler:Enabler".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountIds".into(),
                    value: &account_ids_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypes".into(),
                    value: &resource_types_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EnablerResult {
            account_ids: o.get_field("accountIds"),
            resource_types: o.get_field("resourceTypes"),
        }
    }
}
