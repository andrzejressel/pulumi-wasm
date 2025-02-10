/// Resource for managing an AWS Audit Manager Framework Share.
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
///     let example = framework_share::create(
///         "example",
///         FrameworkShareArgs::builder()
///             .destination_account("123456789012")
///             .destination_region("us-east-1")
///             .framework_id("${exampleAwsAuditmanagerFramework.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Framework Share using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/frameworkShare:FrameworkShare example abcdef-123456
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod framework_share {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrameworkShareArgs {
        /// Comment from the sender about the share request.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Web Services account of the recipient.
        #[builder(into)]
        pub destination_account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Amazon Web Services region of the recipient.
        #[builder(into)]
        pub destination_region: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique identifier for the shared custom framework.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub framework_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FrameworkShareResult {
        /// Comment from the sender about the share request.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Web Services account of the recipient.
        pub destination_account: pulumi_gestalt_rust::Output<String>,
        /// Amazon Web Services region of the recipient.
        pub destination_region: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the shared custom framework.
        ///
        /// The following arguments are optional:
        pub framework_id: pulumi_gestalt_rust::Output<String>,
        /// Status of the share request.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FrameworkShareArgs,
    ) -> FrameworkShareResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let destination_account_binding = args.destination_account.get_output(context);
        let destination_region_binding = args.destination_region.get_output(context);
        let framework_id_binding = args.framework_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/frameworkShare:FrameworkShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationAccount".into(),
                    value: destination_account_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationRegion".into(),
                    value: destination_region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "frameworkId".into(),
                    value: framework_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrameworkShareResult {
            comment: o.get_field("comment"),
            destination_account: o.get_field("destinationAccount"),
            destination_region: o.get_field("destinationRegion"),
            framework_id: o.get_field("frameworkId"),
            status: o.get_field("status"),
        }
    }
}
