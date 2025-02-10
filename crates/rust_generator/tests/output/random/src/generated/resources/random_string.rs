/// The resource `random.RandomString` generates a random permutation of alphanumeric characters and optionally special characters.
///
/// This resource *does* use a cryptographic random number generator.
///
/// Historically this resource's intended usage has been ambiguous as the original example used it in a password. For backwards compatibility it will continue to exist. For unique ids please use random_id, for sensitive random values please use random_password.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let random = random_string::create(
///         "random",
///         RandomStringArgs::builder()
///             .length(16)
///             .override_special("/@£$")
///             .special(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// You can import external strings into your Pulumi programs as RandomString resources as follows:
///
/// ```sh
///  $ import random:index/randomString:RandomString newString myspecialdata
/// ```
///
/// This command will encode the `myspecialdata` token in Pulumi state and generate a code suggestion to include a new RandomString resource in your Pulumi program. Include the suggested code and do a `pulumi up`. Your data is now stored in Pulumi, and you can reference it in your Pulumi program as `newString.result`.
///
/// If the data needs to be stored securily as a secret, consider using the RandomPassword resource instead.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod random_string {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomStringArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
        #[builder(into)]
        pub length: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Include lowercase alphabet characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub lower: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_lower: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum number of numeric characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_numeric: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum number of special characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_special: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
        #[builder(into, default)]
        pub min_upper: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
        #[builder(into, default)]
        pub number: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Include numeric characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub numeric: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
        #[builder(into, default)]
        pub override_special: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
        #[builder(into, default)]
        pub special: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Include uppercase alphabet characters in the result. Default value is `true`.
        #[builder(into, default)]
        pub upper: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct RandomStringResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The length of the string desired. The minimum value for length is 1 and, length must also be >= (`min_upper` + `min_lower` + `min_numeric` + `min_special`).
        pub length: pulumi_gestalt_rust::Output<i32>,
        /// Include lowercase alphabet characters in the result. Default value is `true`.
        pub lower: pulumi_gestalt_rust::Output<bool>,
        /// Minimum number of lowercase alphabet characters in the result. Default value is `0`.
        pub min_lower: pulumi_gestalt_rust::Output<i32>,
        /// Minimum number of numeric characters in the result. Default value is `0`.
        pub min_numeric: pulumi_gestalt_rust::Output<i32>,
        /// Minimum number of special characters in the result. Default value is `0`.
        pub min_special: pulumi_gestalt_rust::Output<i32>,
        /// Minimum number of uppercase alphabet characters in the result. Default value is `0`.
        pub min_upper: pulumi_gestalt_rust::Output<i32>,
        /// Include numeric characters in the result. Default value is `true`. **NOTE**: This is deprecated, use `numeric` instead.
        pub number: pulumi_gestalt_rust::Output<bool>,
        /// Include numeric characters in the result. Default value is `true`.
        pub numeric: pulumi_gestalt_rust::Output<bool>,
        /// Supply your own list of special characters to use for string generation.  This overrides the default character list in the special argument.  The `special` argument must still be set to true for any overwritten characters to be used in generation.
        pub override_special: pulumi_gestalt_rust::Output<Option<String>>,
        /// The generated random string.
        pub result: pulumi_gestalt_rust::Output<String>,
        /// Include special characters in the result. These are `!@#$%&*()-_=+[]{}<>:?`. Default value is `true`.
        pub special: pulumi_gestalt_rust::Output<bool>,
        /// Include uppercase alphabet characters in the result. Default value is `true`.
        pub upper: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RandomStringArgs,
    ) -> RandomStringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let keepers_binding = args.keepers.get_output(context);
        let length_binding = args.length.get_output(context);
        let lower_binding = args.lower.get_output(context);
        let min_lower_binding = args.min_lower.get_output(context);
        let min_numeric_binding = args.min_numeric.get_output(context);
        let min_special_binding = args.min_special.get_output(context);
        let min_upper_binding = args.min_upper.get_output(context);
        let number_binding = args.number.get_output(context);
        let numeric_binding = args.numeric.get_output(context);
        let override_special_binding = args.override_special.get_output(context);
        let special_binding = args.special.get_output(context);
        let upper_binding = args.upper.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "random:index/randomString:RandomString".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepers".into(),
                    value: keepers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "length".into(),
                    value: length_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lower".into(),
                    value: lower_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minLower".into(),
                    value: min_lower_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minNumeric".into(),
                    value: min_numeric_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minSpecial".into(),
                    value: min_special_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minUpper".into(),
                    value: min_upper_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "number".into(),
                    value: number_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numeric".into(),
                    value: numeric_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "overrideSpecial".into(),
                    value: override_special_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "special".into(),
                    value: special_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upper".into(),
                    value: upper_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RandomStringResult {
            keepers: o.get_field("keepers"),
            length: o.get_field("length"),
            lower: o.get_field("lower"),
            min_lower: o.get_field("minLower"),
            min_numeric: o.get_field("minNumeric"),
            min_special: o.get_field("minSpecial"),
            min_upper: o.get_field("minUpper"),
            number: o.get_field("number"),
            numeric: o.get_field("numeric"),
            override_special: o.get_field("overrideSpecial"),
            result: o.get_field("result"),
            special: o.get_field("special"),
            upper: o.get_field("upper"),
        }
    }
}
