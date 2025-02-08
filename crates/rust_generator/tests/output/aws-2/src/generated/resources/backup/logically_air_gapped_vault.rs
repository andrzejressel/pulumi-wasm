/// Resource for managing an AWS Backup Logically Air Gapped Vault.
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
#[allow(clippy::doc_lazy_continuation)]
pub mod logically_air_gapped_vault {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogicallyAirGappedVaultArgs {
        /// Maximum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        #[builder(into)]
        pub max_retention_days: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Minimum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        #[builder(into)]
        pub min_retention_days: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Name of the Logically Air Gapped Backup Vault to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Metadata that you can assign to help organize the resources that you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::backup::LogicallyAirGappedVaultTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct LogicallyAirGappedVaultResult {
        /// The ARN of the Logically Air Gapped Backup Vault.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Maximum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        pub max_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// Minimum retention period that the Logically Air Gapped Backup Vault retains recovery points.
        pub min_retention_days: pulumi_gestalt_rust::Output<i32>,
        /// Name of the Logically Air Gapped Backup Vault to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Metadata that you can assign to help organize the resources that you create. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::backup::LogicallyAirGappedVaultTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LogicallyAirGappedVaultArgs,
    ) -> LogicallyAirGappedVaultResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let max_retention_days_binding = args
            .max_retention_days
            .get_output(context)
            .get_inner();
        let min_retention_days_binding = args
            .min_retention_days
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/logicallyAirGappedVault:LogicallyAirGappedVault".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        LogicallyAirGappedVaultResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            max_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxRetentionDays"),
            ),
            min_retention_days: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minRetentionDays"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
