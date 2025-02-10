/// Provides the ability to register a target with an AWS VPC Lattice Target Group.
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
///     let example = target_group_attachment::create(
///         "example",
///         TargetGroupAttachmentArgs::builder()
///             .target(
///                 TargetGroupAttachmentTarget::builder()
///                     .id("${exampleAwsLb.arn}")
///                     .port(80)
///                     .build_struct(),
///             )
///             .target_group_identifier("${exampleAwsVpclatticeTargetGroup.id}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_group_attachment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentArgs {
        /// The target.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        #[builder(into)]
        pub target_group_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentResult {
        /// The target.
        pub target: pulumi_gestalt_rust::Output<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        pub target_group_identifier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGroupAttachmentArgs,
    ) -> TargetGroupAttachmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let target_binding = args.target.get_output(context);
        let target_group_identifier_binding = args
            .target_group_identifier
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/targetGroupAttachment:TargetGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetGroupIdentifier".into(),
                    value: target_group_identifier_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetGroupAttachmentResult {
            target: o.get_field("target"),
            target_group_identifier: o.get_field("targetGroupIdentifier"),
        }
    }
}
