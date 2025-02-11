#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_random_password {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRandomPasswordArgs {
        /// String of the characters that you don't want in the password.
        #[builder(into, default)]
        pub exclude_characters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to exclude lowercase letters from the password.
        #[builder(into, default)]
        pub exclude_lowercase: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude numbers from the password.
        #[builder(into, default)]
        pub exclude_numbers: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude the following punctuation characters from the password: ``! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~ .``
        #[builder(into, default)]
        pub exclude_punctuation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude uppercase letters from the password.
        #[builder(into, default)]
        pub exclude_uppercase: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to include the space character.
        #[builder(into, default)]
        pub include_space: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Length of the password.
        #[builder(into, default)]
        pub password_length: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation.
        #[builder(into, default)]
        pub require_each_included_type: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetRandomPasswordResult {
        pub exclude_characters: pulumi_gestalt_rust::Output<Option<String>>,
        pub exclude_lowercase: pulumi_gestalt_rust::Output<Option<bool>>,
        pub exclude_numbers: pulumi_gestalt_rust::Output<Option<bool>>,
        pub exclude_punctuation: pulumi_gestalt_rust::Output<Option<bool>>,
        pub exclude_uppercase: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_space: pulumi_gestalt_rust::Output<Option<bool>>,
        pub password_length: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Random password.
        pub random_password: pulumi_gestalt_rust::Output<String>,
        pub require_each_included_type: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRandomPasswordArgs,
    ) -> GetRandomPasswordResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let exclude_characters_binding = args.exclude_characters.get_output(context);
        let exclude_lowercase_binding = args.exclude_lowercase.get_output(context);
        let exclude_numbers_binding = args.exclude_numbers.get_output(context);
        let exclude_punctuation_binding = args.exclude_punctuation.get_output(context);
        let exclude_uppercase_binding = args.exclude_uppercase.get_output(context);
        let include_space_binding = args.include_space.get_output(context);
        let password_length_binding = args.password_length.get_output(context);
        let require_each_included_type_binding = args
            .require_each_included_type
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:secretsmanager/getRandomPassword:getRandomPassword".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeCharacters".into(),
                    value: &exclude_characters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeLowercase".into(),
                    value: &exclude_lowercase_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeNumbers".into(),
                    value: &exclude_numbers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludePunctuation".into(),
                    value: &exclude_punctuation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeUppercase".into(),
                    value: &exclude_uppercase_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeSpace".into(),
                    value: &include_space_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passwordLength".into(),
                    value: &password_length_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requireEachIncludedType".into(),
                    value: &require_each_included_type_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRandomPasswordResult {
            exclude_characters: o.get_field("excludeCharacters"),
            exclude_lowercase: o.get_field("excludeLowercase"),
            exclude_numbers: o.get_field("excludeNumbers"),
            exclude_punctuation: o.get_field("excludePunctuation"),
            exclude_uppercase: o.get_field("excludeUppercase"),
            id: o.get_field("id"),
            include_space: o.get_field("includeSpace"),
            password_length: o.get_field("passwordLength"),
            random_password: o.get_field("randomPassword"),
            require_each_included_type: o.get_field("requireEachIncludedType"),
        }
    }
}
