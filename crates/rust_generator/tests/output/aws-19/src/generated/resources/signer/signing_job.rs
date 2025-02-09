/// Creates a Signer Signing Job.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let buildSigningJob = signing_job::create(
///         "buildSigningJob",
///         SigningJobArgs::builder()
///             .destination(
///                 SigningJobDestination::builder()
///                     .s3(
///                         SigningJobDestinationS3::builder()
///                             .bucket("s3-bucket-name")
///                             .prefix("signed/")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .ignore_signing_job_failure(true)
///             .profile_name("${testSp.name}")
///             .source(
///                 SigningJobSource::builder()
///                     .s3(
///                         SigningJobSourceS3::builder()
///                             .bucket("s3-bucket-name")
///                             .key("object-to-be-signed.zip")
///                             .version("jADjFYYYEXAMPLETszPjOmCMFDzd9dN1")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let testSp = signing_profile::create(
///         "testSp",
///         SigningProfileArgs::builder()
///             .platform_id("AWSLambda-SHA384-ECDSA")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Signer signing jobs using the `job_id`. For example:
///
/// ```sh
/// $ pulumi import aws:signer/signingJob:SigningJob test_signer_signing_job 9ed7e5c3-b8d4-4da0-8459-44e0b068f7ee
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod signing_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SigningJobArgs {
        /// The S3 bucket in which to save your signed object. See Destination below for details.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::signer::SigningJobDestination,
        >,
        /// Set this argument to `true` to ignore signing job failures and retrieve failed status and reason. Default `false`.
        #[builder(into, default)]
        pub ignore_signing_job_failure: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the profile to initiate the signing operation.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The S3 bucket that contains the object to sign. See Source below for details.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::signer::SigningJobSource,
        >,
    }
    #[allow(dead_code)]
    pub struct SigningJobResult {
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was completed.
        pub completed_at: pulumi_gestalt_rust::Output<String>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the signing job was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The S3 bucket in which to save your signed object. See Destination below for details.
        pub destination: pulumi_gestalt_rust::Output<
            super::super::types::signer::SigningJobDestination,
        >,
        /// Set this argument to `true` to ignore signing job failures and retrieve failed status and reason. Default `false`.
        pub ignore_signing_job_failure: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the signing job on output.
        pub job_id: pulumi_gestalt_rust::Output<String>,
        /// The IAM entity that initiated the signing job.
        pub job_invoker: pulumi_gestalt_rust::Output<String>,
        /// The AWS account ID of the job owner.
        pub job_owner: pulumi_gestalt_rust::Output<String>,
        /// A human-readable name for the signing platform associated with the signing job.
        pub platform_display_name: pulumi_gestalt_rust::Output<String>,
        /// The platform to which your signed code image will be distributed.
        pub platform_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the profile to initiate the signing operation.
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        /// The version of the signing profile used to initiate the signing job.
        pub profile_version: pulumi_gestalt_rust::Output<String>,
        /// The IAM principal that requested the signing job.
        pub requested_by: pulumi_gestalt_rust::Output<String>,
        /// A revocation record if the signature generated by the signing job has been revoked. Contains a timestamp and the ID of the IAM entity that revoked the signature.
        pub revocation_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::signer::SigningJobRevocationRecord>,
        >,
        /// The time when the signature of a signing job expires.
        pub signature_expires_at: pulumi_gestalt_rust::Output<String>,
        /// Name of the S3 bucket where the signed code image is saved by code signing.
        pub signed_objects: pulumi_gestalt_rust::Output<
            Vec<super::super::types::signer::SigningJobSignedObject>,
        >,
        /// The S3 bucket that contains the object to sign. See Source below for details.
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::signer::SigningJobSource,
        >,
        /// Status of the signing job.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// String value that contains the status reason.
        pub status_reason: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SigningJobArgs,
    ) -> SigningJobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_binding = args.destination.get_output(context);
        let ignore_signing_job_failure_binding = args
            .ignore_signing_job_failure
            .get_output(context);
        let profile_name_binding = args.profile_name.get_output(context);
        let source_binding = args.source.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:signer/signingJob:SigningJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreSigningJobFailure".into(),
                    value: ignore_signing_job_failure_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileName".into(),
                    value: profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SigningJobResult {
            completed_at: o.get_field("completedAt"),
            created_at: o.get_field("createdAt"),
            destination: o.get_field("destination"),
            ignore_signing_job_failure: o.get_field("ignoreSigningJobFailure"),
            job_id: o.get_field("jobId"),
            job_invoker: o.get_field("jobInvoker"),
            job_owner: o.get_field("jobOwner"),
            platform_display_name: o.get_field("platformDisplayName"),
            platform_id: o.get_field("platformId"),
            profile_name: o.get_field("profileName"),
            profile_version: o.get_field("profileVersion"),
            requested_by: o.get_field("requestedBy"),
            revocation_records: o.get_field("revocationRecords"),
            signature_expires_at: o.get_field("signatureExpiresAt"),
            signed_objects: o.get_field("signedObjects"),
            source: o.get_field("source"),
            status: o.get_field("status"),
            status_reason: o.get_field("statusReason"),
        }
    }
}
