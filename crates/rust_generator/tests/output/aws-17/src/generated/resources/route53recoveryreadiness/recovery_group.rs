/// Provides an AWS Route 53 Recovery Readiness Recovery Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = recovery_group::create(
///         "example",
///         RecoveryGroupArgs::builder()
///             .recovery_group_name("my-high-availability-app")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Recovery Readiness recovery groups using the recovery group name. For example:
///
/// ```sh
/// $ pulumi import aws:route53recoveryreadiness/recoveryGroup:RecoveryGroup my-high-availability-app my-high-availability-app
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod recovery_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecoveryGroupArgs {
        /// List of cell arns to add as nested fault domains within this recovery group
        #[builder(into, default)]
        pub cells: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A unique name describing the recovery group.
        ///
        /// The following argument are optional:
        #[builder(into)]
        pub recovery_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RecoveryGroupResult {
        /// ARN of the recovery group
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// List of cell arns to add as nested fault domains within this recovery group
        pub cells: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A unique name describing the recovery group.
        ///
        /// The following argument are optional:
        pub recovery_group_name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecoveryGroupArgs,
    ) -> RecoveryGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cells_binding = args.cells.get_output(context);
        let recovery_group_name_binding = args.recovery_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/recoveryGroup:RecoveryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cells".into(),
                    value: cells_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recoveryGroupName".into(),
                    value: recovery_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecoveryGroupResult {
            arn: o.get_field("arn"),
            cells: o.get_field("cells"),
            recovery_group_name: o.get_field("recoveryGroupName"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
