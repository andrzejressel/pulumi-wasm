pub mod get_random_password {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRandomPasswordArgs {
        /// String of the characters that you don't want in the password.
        #[builder(into, default)]
        pub exclude_characters: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies whether to exclude lowercase letters from the password.
        #[builder(into, default)]
        pub exclude_lowercase: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude numbers from the password.
        #[builder(into, default)]
        pub exclude_numbers: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude the following punctuation characters from the password: ``! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~ .``
        #[builder(into, default)]
        pub exclude_punctuation: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to exclude uppercase letters from the password.
        #[builder(into, default)]
        pub exclude_uppercase: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to include the space character.
        #[builder(into, default)]
        pub include_space: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Length of the password.
        #[builder(into, default)]
        pub password_length: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Specifies whether to include at least one upper and lowercase letter, one number, and one punctuation.
        #[builder(into, default)]
        pub require_each_included_type: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetRandomPasswordResult {
        pub exclude_characters: pulumi_wasm_rust::Output<Option<String>>,
        pub exclude_lowercase: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_numbers: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_punctuation: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_uppercase: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_space: pulumi_wasm_rust::Output<Option<bool>>,
        pub password_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// Random password.
        pub random_password: pulumi_wasm_rust::Output<String>,
        pub require_each_included_type: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetRandomPasswordArgs,
    ) -> GetRandomPasswordResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let exclude_characters_binding = args
            .exclude_characters
            .get_output(context)
            .get_inner();
        let exclude_lowercase_binding = args
            .exclude_lowercase
            .get_output(context)
            .get_inner();
        let exclude_numbers_binding = args
            .exclude_numbers
            .get_output(context)
            .get_inner();
        let exclude_punctuation_binding = args
            .exclude_punctuation
            .get_output(context)
            .get_inner();
        let exclude_uppercase_binding = args
            .exclude_uppercase
            .get_output(context)
            .get_inner();
        let include_space_binding = args.include_space.get_output(context).get_inner();
        let password_length_binding = args
            .password_length
            .get_output(context)
            .get_inner();
        let require_each_included_type_binding = args
            .require_each_included_type
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:secretsmanager/getRandomPassword:getRandomPassword".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "excludeCharacters".into(),
                    value: &exclude_characters_binding,
                },
                register_interface::ObjectField {
                    name: "excludeLowercase".into(),
                    value: &exclude_lowercase_binding,
                },
                register_interface::ObjectField {
                    name: "excludeNumbers".into(),
                    value: &exclude_numbers_binding,
                },
                register_interface::ObjectField {
                    name: "excludePunctuation".into(),
                    value: &exclude_punctuation_binding,
                },
                register_interface::ObjectField {
                    name: "excludeUppercase".into(),
                    value: &exclude_uppercase_binding,
                },
                register_interface::ObjectField {
                    name: "includeSpace".into(),
                    value: &include_space_binding,
                },
                register_interface::ObjectField {
                    name: "passwordLength".into(),
                    value: &password_length_binding,
                },
                register_interface::ObjectField {
                    name: "requireEachIncludedType".into(),
                    value: &require_each_included_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRandomPasswordResult {
            exclude_characters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeCharacters"),
            ),
            exclude_lowercase: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeLowercase"),
            ),
            exclude_numbers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeNumbers"),
            ),
            exclude_punctuation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludePunctuation"),
            ),
            exclude_uppercase: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeUppercase"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            include_space: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includeSpace"),
            ),
            password_length: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("passwordLength"),
            ),
            random_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("randomPassword"),
            ),
            require_each_included_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("requireEachIncludedType"),
            ),
        }
    }
}
