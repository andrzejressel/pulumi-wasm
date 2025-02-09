/// The hmacKeys resource represents an HMAC key within Cloud Storage. The resource
/// consists of a secret and HMAC key metadata. HMAC keys can be used as credentials
/// for service accounts.
///
///
/// To get more information about HmacKey, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/projects/hmacKeys)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/authentication/managing-hmackeys)
///
///
///
/// ## Example Usage
///
/// ### Storage Hmac Key
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let key = hmac_key::create(
///         "key",
///         HmacKeyArgs::builder()
///             .service_account_email("${serviceAccount.email}")
///             .build_struct(),
///     );
///     let serviceAccount = account::create(
///         "serviceAccount",
///         AccountArgs::builder().account_id("my-svc-acc").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HmacKey can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/hmacKeys/{{access_id}}`
///
/// * `{{project}}/{{access_id}}`
///
/// * `{{access_id}}`
///
/// When using the `pulumi import` command, HmacKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default projects/{{project}}/hmacKeys/{{access_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default {{project}}/{{access_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default {{access_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hmac_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HmacKeyArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The email address of the key's associated service account.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_account_email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The state of the key. Can be set to one of ACTIVE, INACTIVE.
        /// Default value is `ACTIVE`.
        /// Possible values are: `ACTIVE`, `INACTIVE`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HmacKeyResult {
        /// The access ID of the HMAC Key.
        pub access_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// HMAC secret key material.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret: pulumi_gestalt_rust::Output<String>,
        /// The email address of the key's associated service account.
        ///
        ///
        /// - - -
        pub service_account_email: pulumi_gestalt_rust::Output<String>,
        /// The state of the key. Can be set to one of ACTIVE, INACTIVE.
        /// Default value is `ACTIVE`.
        /// Possible values are: `ACTIVE`, `INACTIVE`.
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// 'The creation time of the HMAC key in RFC 3339 format. '
        pub time_created: pulumi_gestalt_rust::Output<String>,
        /// 'The last modification time of the HMAC key metadata in RFC 3339 format.'
        pub updated: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HmacKeyArgs,
    ) -> HmacKeyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let service_account_email_binding = args
            .service_account_email
            .get_output(context);
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:storage/hmacKey:HmacKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccountEmail".into(),
                    value: service_account_email_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HmacKeyResult {
            access_id: o.get_field("accessId"),
            project: o.get_field("project"),
            secret: o.get_field("secret"),
            service_account_email: o.get_field("serviceAccountEmail"),
            state: o.get_field("state"),
            time_created: o.get_field("timeCreated"),
            updated: o.get_field("updated"),
        }
    }
}
