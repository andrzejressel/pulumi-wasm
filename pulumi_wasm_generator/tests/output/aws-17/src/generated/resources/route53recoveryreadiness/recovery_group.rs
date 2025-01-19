/// Provides an AWS Route 53 Recovery Readiness Recovery Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod recovery_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecoveryGroupArgs {
        /// List of cell arns to add as nested fault domains within this recovery group
        #[builder(into, default)]
        pub cells: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A unique name describing the recovery group.
        ///
        /// The following argument are optional:
        #[builder(into)]
        pub recovery_group_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RecoveryGroupResult {
        /// ARN of the recovery group
        pub arn: pulumi_wasm_rust::Output<String>,
        /// List of cell arns to add as nested fault domains within this recovery group
        pub cells: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A unique name describing the recovery group.
        ///
        /// The following argument are optional:
        pub recovery_group_name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RecoveryGroupArgs) -> RecoveryGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cells_binding = args.cells.get_inner();
        let recovery_group_name_binding = args.recovery_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53recoveryreadiness/recoveryGroup:RecoveryGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cells".into(),
                    value: &cells_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryGroupName".into(),
                    value: &recovery_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "cells".into(),
                },
                register_interface::ResultField {
                    name: "recoveryGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RecoveryGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cells: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cells").unwrap(),
            ),
            recovery_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
