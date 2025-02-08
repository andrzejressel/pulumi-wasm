/// Provides a resource which manages Cloudflare Logpush ownership
/// challenges to use in a Logpush Job. On it's own, doesn't do much
/// however this resource should be used in conjunction to create
/// Logpush jobs.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = logpush_ownership_challenge::create(
///         "example",
///         LogpushOwnershipChallengeArgs::builder()
///             .destination_conf("s3://my-bucket-path?region=us-west-2")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod logpush_ownership_challenge {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogpushOwnershipChallengeArgs {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub destination_conf: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogpushOwnershipChallengeResult {
        /// The account identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Uniquely identifies a resource (such as an s3 bucket) where data will be pushed. Additional configuration parameters supported by the destination may be included. See [Logpush destination documentation](https://developers.cloudflare.com/logs/logpush/logpush-configuration-api/understanding-logpush-api/#destination). **Modifying this attribute will force creation of a new resource.**
        pub destination_conf: pulumi_gestalt_rust::Output<String>,
        /// The filename of the ownership challenge which	contains the contents required for Logpush Job creation.
        pub ownership_challenge_filename: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Must provide only one of `account_id`, `zone_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LogpushOwnershipChallengeArgs,
    ) -> LogpushOwnershipChallengeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let destination_conf_binding = args
            .destination_conf
            .get_output(context)
            .get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/logpushOwnershipChallenge:LogpushOwnershipChallenge"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "destinationConf".into(),
                    value: &destination_conf_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LogpushOwnershipChallengeResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            destination_conf: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationConf"),
            ),
            ownership_challenge_filename: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownershipChallengeFilename"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
