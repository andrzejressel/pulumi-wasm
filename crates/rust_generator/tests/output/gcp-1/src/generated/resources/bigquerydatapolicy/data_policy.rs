/// A BigQuery Data Policy
///
///
/// To get more information about DataPolicy, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/bigquerydatapolicy/rest/v1beta1/projects.locations.dataPolicies/create)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/bigquery/docs/column-data-masking-intro)
///
/// ## Example Usage
///
/// ### Bigquery Datapolicy Data Policy Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dataPolicy = data_policy::create(
///         "dataPolicy",
///         DataPolicyArgs::builder()
///             .data_policy_id("data_policy")
///             .data_policy_type("COLUMN_LEVEL_SECURITY_POLICY")
///             .location("us-central1")
///             .policy_tag("${policyTag.name}")
///             .build_struct(),
///     );
///     let policyTag = policy_tag::create(
///         "policyTag",
///         PolicyTagArgs::builder()
///             .description("A policy tag normally associated with low security items")
///             .display_name("Low security")
///             .taxonomy("${taxonomy.id}")
///             .build_struct(),
///     );
///     let taxonomy = taxonomy::create(
///         "taxonomy",
///         TaxonomyArgs::builder()
///             .activated_policy_types(vec!["FINE_GRAINED_ACCESS_CONTROL",])
///             .description("A collection of policy tags")
///             .display_name("taxonomy")
///             .region("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Bigquery Datapolicy Data Policy Routine
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let customMaskingRoutine = routine::create(
///         "customMaskingRoutine",
///         RoutineArgs::builder()
///             .arguments(
///                 vec![
///                     RoutineArgument::builder().dataType("{\"typeKind\" :  \"STRING\"}")
///                     .name("ssn").build_struct(),
///                 ],
///             )
///             .data_governance_type("DATA_MASKING")
///             .dataset_id("${test.datasetId}")
///             .definition_body("SAFE.REGEXP_REPLACE(ssn, '[0-9]', 'X')")
///             .language("SQL")
///             .return_type("{\"typeKind\" :  \"STRING\"}")
///             .routine_id("custom_masking_routine")
///             .routine_type("SCALAR_FUNCTION")
///             .build_struct(),
///     );
///     let dataPolicy = data_policy::create(
///         "dataPolicy",
///         DataPolicyArgs::builder()
///             .data_masking_policy(
///                 DataPolicyDataMaskingPolicy::builder()
///                     .routine("${customMaskingRoutine.id}")
///                     .build_struct(),
///             )
///             .data_policy_id("data_policy")
///             .data_policy_type("DATA_MASKING_POLICY")
///             .location("us-central1")
///             .policy_tag("${policyTag.name}")
///             .build_struct(),
///     );
///     let policyTag = policy_tag::create(
///         "policyTag",
///         PolicyTagArgs::builder()
///             .description("A policy tag normally associated with low security items")
///             .display_name("Low security")
///             .taxonomy("${taxonomy.id}")
///             .build_struct(),
///     );
///     let taxonomy = taxonomy::create(
///         "taxonomy",
///         TaxonomyArgs::builder()
///             .activated_policy_types(vec!["FINE_GRAINED_ACCESS_CONTROL",])
///             .description("A collection of policy tags")
///             .display_name("taxonomy")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let test = dataset::create(
///         "test",
///         DatasetArgs::builder()
///             .dataset_id("dataset_id")
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DataPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/dataPolicies/{{data_policy_id}}`
///
/// * `{{project}}/{{location}}/{{data_policy_id}}`
///
/// * `{{location}}/{{data_policy_id}}`
///
/// When using the `pulumi import` command, DataPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquerydatapolicy/dataPolicy:DataPolicy default projects/{{project}}/locations/{{location}}/dataPolicies/{{data_policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquerydatapolicy/dataPolicy:DataPolicy default {{project}}/{{location}}/{{data_policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquerydatapolicy/dataPolicy:DataPolicy default {{location}}/{{data_policy_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod data_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataPolicyArgs {
        /// The data masking policy that specifies the data masking rule to use.
        /// Structure is documented below.
        #[builder(into, default)]
        pub data_masking_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquerydatapolicy::DataPolicyDataMaskingPolicy>,
        >,
        /// User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {dataPolicyId} in part of the resource name.
        #[builder(into)]
        pub data_policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The enrollment level of the service.
        /// Possible values are: `COLUMN_LEVEL_SECURITY_POLICY`, `DATA_MASKING_POLICY`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub data_policy_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the location of the data policy.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Policy tag resource name, in the format of projects/{project_number}/locations/{locationId}/taxonomies/{taxonomyId}/policyTags/{policyTag_id}.
        #[builder(into)]
        pub policy_tag: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DataPolicyResult {
        /// The data masking policy that specifies the data masking rule to use.
        /// Structure is documented below.
        pub data_masking_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquerydatapolicy::DataPolicyDataMaskingPolicy>,
        >,
        /// User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {dataPolicyId} in part of the resource name.
        pub data_policy_id: pulumi_gestalt_rust::Output<String>,
        /// The enrollment level of the service.
        /// Possible values are: `COLUMN_LEVEL_SECURITY_POLICY`, `DATA_MASKING_POLICY`.
        ///
        ///
        /// - - -
        pub data_policy_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the location of the data policy.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Resource name of this data policy, in the format of projects/{project_number}/locations/{locationId}/dataPolicies/{dataPolicyId}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Policy tag resource name, in the format of projects/{project_number}/locations/{locationId}/taxonomies/{taxonomyId}/policyTags/{policyTag_id}.
        pub policy_tag: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DataPolicyArgs,
    ) -> DataPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_masking_policy_binding = args.data_masking_policy.get_output(context);
        let data_policy_id_binding = args.data_policy_id.get_output(context);
        let data_policy_type_binding = args.data_policy_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let policy_tag_binding = args.policy_tag.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquerydatapolicy/dataPolicy:DataPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataMaskingPolicy".into(),
                    value: &data_masking_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataPolicyId".into(),
                    value: &data_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataPolicyType".into(),
                    value: &data_policy_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyTag".into(),
                    value: &policy_tag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DataPolicyResult {
            data_masking_policy: o.get_field("dataMaskingPolicy"),
            data_policy_id: o.get_field("dataPolicyId"),
            data_policy_type: o.get_field("dataPolicyType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy_tag: o.get_field("policyTag"),
            project: o.get_field("project"),
        }
    }
}
