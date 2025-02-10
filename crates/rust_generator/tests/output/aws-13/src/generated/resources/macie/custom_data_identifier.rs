/// Provides a resource to manage an [AWS Macie Custom Data Identifier](https://docs.aws.amazon.com/macie/latest/APIReference/custom-data-identifiers-id.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account::create("example", AccountArgs::builder().build_struct());
///     let exampleCustomDataIdentifier = custom_data_identifier::create(
///         "exampleCustomDataIdentifier",
///         CustomDataIdentifierArgs::builder()
///             .description("DESCRIPTION")
///             .ignore_words(vec!["ignore",])
///             .keywords(vec!["keyword",])
///             .maximum_match_distance(10)
///             .name("NAME OF CUSTOM DATA IDENTIFIER")
///             .regex("[0-9]{3}-[0-9]{2}-[0-9]{4}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_macie2_custom_data_identifier` using the id. For example:
///
/// ```sh
/// $ pulumi import aws:macie/customDataIdentifier:CustomDataIdentifier example abcd1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_data_identifier {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomDataIdentifierArgs {
        /// A custom description of the custom data identifier. The description can contain as many as 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array that lists specific character sequences (ignore words) to exclude from the results. If the text matched by the regular expression is the same as any string in this array, Amazon Macie ignores it. The array can contain as many as 10 ignore words. Each ignore word can contain 4 - 90 characters. Ignore words are case sensitive.
        #[builder(into, default)]
        pub ignore_words: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An array that lists specific character sequences (keywords), one of which must be within proximity (`maximum_match_distance`) of the regular expression to match. The array can contain as many as 50 keywords. Each keyword can contain 3 - 90 characters. Keywords aren't case sensitive.
        #[builder(into, default)]
        pub keywords: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The maximum number of characters that can exist between text that matches the regex pattern and the character sequences specified by the keywords array. Macie includes or excludes a result based on the proximity of a keyword to text that matches the regex pattern. The distance can be 1 - 300 characters. The default value is 50.
        #[builder(into, default)]
        pub maximum_match_distance: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A custom name for the custom data identifier. The name can contain as many as 128 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The regular expression (regex) that defines the pattern to match. The expression can contain as many as 512 characters.
        #[builder(into, default)]
        pub regex: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of key-value pairs that specifies the tags to associate with the custom data identifier.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomDataIdentifierResult {
        /// The Amazon Resource Name (ARN) of the custom data identifier.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The date and time, in UTC and extended RFC 3339 format, when the Amazon Macie account was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// A custom description of the custom data identifier. The description can contain as many as 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An array that lists specific character sequences (ignore words) to exclude from the results. If the text matched by the regular expression is the same as any string in this array, Amazon Macie ignores it. The array can contain as many as 10 ignore words. Each ignore word can contain 4 - 90 characters. Ignore words are case sensitive.
        pub ignore_words: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An array that lists specific character sequences (keywords), one of which must be within proximity (`maximum_match_distance`) of the regular expression to match. The array can contain as many as 50 keywords. Each keyword can contain 3 - 90 characters. Keywords aren't case sensitive.
        pub keywords: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The maximum number of characters that can exist between text that matches the regex pattern and the character sequences specified by the keywords array. Macie includes or excludes a result based on the proximity of a keyword to text that matches the regex pattern. The distance can be 1 - 300 characters. The default value is 50.
        pub maximum_match_distance: pulumi_gestalt_rust::Output<i32>,
        /// A custom name for the custom data identifier. The name can contain as many as 128 characters. If omitted, the provider will assign a random, unique name. Conflicts with `name_prefix`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the specified prefix. Conflicts with `name`.
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The regular expression (regex) that defines the pattern to match. The expression can contain as many as 512 characters.
        pub regex: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of key-value pairs that specifies the tags to associate with the custom data identifier.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: CustomDataIdentifierArgs,
    ) -> CustomDataIdentifierResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let ignore_words_binding = args.ignore_words.get_output(context);
        let keywords_binding = args.keywords.get_output(context);
        let maximum_match_distance_binding = args
            .maximum_match_distance
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let regex_binding = args.regex.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:macie/customDataIdentifier:CustomDataIdentifier".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreWords".into(),
                    value: ignore_words_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keywords".into(),
                    value: keywords_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumMatchDistance".into(),
                    value: maximum_match_distance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regex".into(),
                    value: regex_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomDataIdentifierResult {
            arn: o.get_field("arn"),
            created_at: o.get_field("createdAt"),
            description: o.get_field("description"),
            ignore_words: o.get_field("ignoreWords"),
            keywords: o.get_field("keywords"),
            maximum_match_distance: o.get_field("maximumMatchDistance"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            regex: o.get_field("regex"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
