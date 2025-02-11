/// Provides a resource to manage an S3 Multi-Region Access Point access control policy.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Multi-Region Access Point Policies using the `account_id` and `name` of the Multi-Region Access Point separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy example 123456789012:example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod multi_region_access_point_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyArgs {
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        #[builder(into)]
        pub details: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
    }
    #[allow(dead_code)]
    pub struct MultiRegionAccessPointPolicyResult {
        /// The AWS account ID for the owner of the Multi-Region Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block containing details about the policy for the Multi-Region Access Point. See Details Configuration Block below for more details
        pub details: pulumi_gestalt_rust::Output<
            super::super::types::s3control::MultiRegionAccessPointPolicyDetails,
        >,
        /// The last established policy for the Multi-Region Access Point.
        pub established: pulumi_gestalt_rust::Output<String>,
        /// The proposed policy for the Multi-Region Access Point.
        pub proposed: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MultiRegionAccessPointPolicyArgs,
    ) -> MultiRegionAccessPointPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let details_binding = args.details.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:s3control/multiRegionAccessPointPolicy:MultiRegionAccessPointPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "details".into(),
                    value: &details_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MultiRegionAccessPointPolicyResult {
            account_id: o.get_field("accountId"),
            details: o.get_field("details"),
            established: o.get_field("established"),
            proposed: o.get_field("proposed"),
        }
    }
}
