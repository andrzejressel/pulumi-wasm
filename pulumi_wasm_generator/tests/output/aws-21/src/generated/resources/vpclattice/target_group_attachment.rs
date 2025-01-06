/// Provides the ability to register a target with an AWS VPC Lattice Target Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod target_group_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentArgs {
        /// The target.
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        #[builder(into)]
        pub target_group_identifier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupAttachmentResult {
        /// The target.
        pub target: pulumi_wasm_rust::Output<
            super::super::types::vpclattice::TargetGroupAttachmentTarget,
        >,
        /// The ID or Amazon Resource Name (ARN) of the target group.
        pub target_group_identifier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: TargetGroupAttachmentArgs,
    ) -> TargetGroupAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let target_binding = args.target.get_inner();
        let target_group_identifier_binding = args.target_group_identifier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:vpclattice/targetGroupAttachment:TargetGroupAttachment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "targetGroupIdentifier".into(),
                    value: &target_group_identifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "targetGroupIdentifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TargetGroupAttachmentResult {
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            target_group_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetGroupIdentifier").unwrap(),
            ),
        }
    }
}
