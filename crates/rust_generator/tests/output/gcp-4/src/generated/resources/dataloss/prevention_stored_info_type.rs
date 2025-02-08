/// Allows creation of custom info types.
///
///
/// To get more information about StoredInfoType, see:
///
/// * [API documentation](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.storedInfoTypes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dlp/docs/creating-stored-infotypes)
///
/// ## Example Usage
///
/// ### Dlp Stored Info Type Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_stored_info_type::create(
///         "basic",
///         PreventionStoredInfoTypeArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .parent("projects/my-project-name")
///             .regex(
///                 PreventionStoredInfoTypeRegex::builder()
///                     .groupIndexes(vec![2,])
///                     .pattern("patient")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Stored Info Type Dictionary
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dictionary = prevention_stored_info_type::create(
///         "dictionary",
///         PreventionStoredInfoTypeArgs::builder()
///             .description("Description")
///             .dictionary(
///                 PreventionStoredInfoTypeDictionary::builder()
///                     .wordList(
///                         PreventionStoredInfoTypeDictionaryWordList::builder()
///                             .words(vec!["word", "word2",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .display_name("Displayname")
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Stored Info Type Large Custom Dictionary
///
///
/// ```yaml
/// resources:
///   large:
///     type: gcp:dataloss:PreventionStoredInfoType
///     properties:
///       parent: projects/my-project-name
///       description: Description
///       displayName: Displayname
///       largeCustomDictionary:
///         cloudStorageFileSet:
///           url: gs://${bucket.name}/${object.name}
///         outputPath:
///           path: gs://${bucket.name}/output/dictionary.txt
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: tf-test-bucket
///       location: US
///       forceDestroy: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: tf-test-object
///       bucket: ${bucket.name}
///       source:
///         fn::FileAsset: ./test-fixtures/words.txt
/// ```
/// ### Dlp Stored Info Type With Id
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let withStoredInfoTypeId = prevention_stored_info_type::create(
///         "withStoredInfoTypeId",
///         PreventionStoredInfoTypeArgs::builder()
///             .description("Description")
///             .display_name("Displayname")
///             .parent("projects/my-project-name")
///             .regex(
///                 PreventionStoredInfoTypeRegex::builder()
///                     .groupIndexes(vec![2,])
///                     .pattern("patient")
///                     .build_struct(),
///             )
///             .stored_info_type_id("id-")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// StoredInfoType can be imported using any of these accepted formats:
///
/// * `{{parent}}/storedInfoTypes/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, StoredInfoType can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionStoredInfoType:PreventionStoredInfoType default {{parent}}/storedInfoTypes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionStoredInfoType:PreventionStoredInfoType default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prevention_stored_info_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreventionStoredInfoTypeArgs {
        /// A description of the info type.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Dictionary which defines the rule.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dictionary: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataloss::PreventionStoredInfoTypeDictionary>,
        >,
        /// User set display name of the info type.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Dictionary which defines the rule.
        /// Structure is documented below.
        #[builder(into, default)]
        pub large_custom_dictionary: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionary,
            >,
        >,
        /// The parent of the info type in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Regular expression which defines the rule.
        /// Structure is documented below.
        #[builder(into, default)]
        pub regex: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataloss::PreventionStoredInfoTypeRegex>,
        >,
        /// The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;
        /// that is, it must match the regular expression: [a-zA-Z\d-_]+. The maximum length is 100
        /// characters. Can be empty to allow the system to generate one.
        #[builder(into, default)]
        pub stored_info_type_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PreventionStoredInfoTypeResult {
        /// A description of the info type.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Dictionary which defines the rule.
        /// Structure is documented below.
        pub dictionary: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataloss::PreventionStoredInfoTypeDictionary>,
        >,
        /// User set display name of the info type.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Dictionary which defines the rule.
        /// Structure is documented below.
        pub large_custom_dictionary: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionary,
            >,
        >,
        /// The resource name of the info type. Set by the server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the info type in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Regular expression which defines the rule.
        /// Structure is documented below.
        pub regex: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataloss::PreventionStoredInfoTypeRegex>,
        >,
        /// The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;
        /// that is, it must match the regular expression: [a-zA-Z\d-_]+. The maximum length is 100
        /// characters. Can be empty to allow the system to generate one.
        pub stored_info_type_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PreventionStoredInfoTypeArgs,
    ) -> PreventionStoredInfoTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let dictionary_binding = args.dictionary.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let large_custom_dictionary_binding = args
            .large_custom_dictionary
            .get_output(context)
            .get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let regex_binding = args.regex.get_output(context).get_inner();
        let stored_info_type_id_binding = args
            .stored_info_type_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataloss/preventionStoredInfoType:PreventionStoredInfoType"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dictionary".into(),
                    value: &dictionary_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "largeCustomDictionary".into(),
                    value: &large_custom_dictionary_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "regex".into(),
                    value: &regex_binding,
                },
                register_interface::ObjectField {
                    name: "storedInfoTypeId".into(),
                    value: &stored_info_type_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PreventionStoredInfoTypeResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dictionary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dictionary"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            large_custom_dictionary: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("largeCustomDictionary"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            regex: pulumi_gestalt_rust::__private::into_domain(o.extract_field("regex")),
            stored_info_type_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storedInfoTypeId"),
            ),
        }
    }
}
