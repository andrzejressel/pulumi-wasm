/// Resource for managing an AWS Backup Logically Air Gapped Vault.
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
///     let example = logically_air_gapped_vault::create(
///         "example",
///         LogicallyAirGappedVaultArgs::builder()
///             .max_retention_days(7)
///             .min_retention_days(7)
///             .name("lag-example-vault")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Logically Air Gapped Vault using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/logicallyAirGappedVault:LogicallyAirGappedVault example lag-example-vault
/// ```
pub mod logically_air_gapped_vault {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogicallyAirGappedVaultArgs {
        /// Maximum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        #[builder(into)]
        pub max_retention_days: pulumi_wasm_rust::Output<i32>,
        /// Minimum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        #[builder(into)]
        pub min_retention_days: pulumi_wasm_rust::Output<i32>,
        /// Name of the Logically Air Gapped Backup Vault to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Metadata that you can assign to help organize the resources that you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::backup::LogicallyAirGappedVaultTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct LogicallyAirGappedVaultResult {
        /// The ARN of the Logically Air Gapped Backup Vault.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Maximum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        pub max_retention_days: pulumi_wasm_rust::Output<i32>,
        /// Minimum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        pub min_retention_days: pulumi_wasm_rust::Output<i32>,
        /// Name of the Logically Air Gapped Backup Vault to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Metadata that you can assign to help organize the resources that you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::backup::LogicallyAirGappedVaultTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: LogicallyAirGappedVaultArgs,
    ) -> LogicallyAirGappedVaultResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let max_retention_days_binding = args.max_retention_days.get_inner();
        let min_retention_days_binding = args.min_retention_days.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/logicallyAirGappedVault:LogicallyAirGappedVault".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "maxRetentionDays".into(),
                    value: &max_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "minRetentionDays".into(),
                    value: &min_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "maxRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "minRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LogicallyAirGappedVaultResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            max_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRetentionDays").unwrap(),
            ),
            min_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minRetentionDays").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}