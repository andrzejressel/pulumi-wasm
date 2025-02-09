/// Provides an Elastic Beanstalk Application Version Resource. Elastic Beanstalk allows
/// you to deploy and manage applications in the AWS cloud without worrying about
/// the infrastructure that runs those applications.
///
/// This resource creates a Beanstalk Application Version that can be deployed to a Beanstalk
/// Environment.
///
/// > **NOTE on Application Version Resource:**  When using the Application Version resource with multiple
/// Elastic Beanstalk Environments it is possible that an error may be returned
/// when attempting to delete an Application Version while it is still in use by a different environment.
/// To work around this you can either create each environment in a separate AWS account or create your `aws.elasticbeanstalk.ApplicationVersion` resources with a unique names in your Elastic Beanstalk Application. For example &lt;revision&gt;-&lt;environment&gt;.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: tftest.applicationversion.bucket
///   defaultBucketObjectv2:
///     type: aws:s3:BucketObjectv2
///     name: default
///     properties:
///       bucket: ${default.id}
///       key: beanstalk/go-v1.zip
///       source:
///         fn::FileAsset: go-v1.zip
///   defaultApplication:
///     type: aws:elasticbeanstalk:Application
///     name: default
///     properties:
///       name: tf-test-name
///       description: tf-test-desc
///   defaultApplicationVersion:
///     type: aws:elasticbeanstalk:ApplicationVersion
///     name: default
///     properties:
///       name: tf-test-version-label
///       application: tf-test-name
///       description: application version
///       bucket: ${default.id}
///       key: ${defaultBucketObjectv2.id}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationVersionArgs {
        /// Name of the Beanstalk Application the version is associated with.
        #[builder(into)]
        pub application: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 bucket that contains the Application Version source bundle.
        #[builder(into)]
        pub bucket: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Short description of the Application Version.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// On delete, force an Application Version to be deleted when it may be in use by multiple Elastic Beanstalk Environments.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// S3 object that is the Application Version source bundle.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Unique name for the this Application Version.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Pre-processes and validates the environment manifest (env.yaml ) and configuration files (*.config files in the .ebextensions folder) in the source bundle. Validating configuration files can identify issues prior to deploying the application version to an environment. You must turn processing on for application versions that you create using AWS CodeBuild or AWS CodeCommit. For application versions built from a source bundle in Amazon S3, processing is optional. It validates Elastic Beanstalk configuration files. It doesn’t validate your application’s configuration files, like proxy server or Docker configuration.
        #[builder(into, default)]
        pub process: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of tags for the Elastic Beanstalk Application Version. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationVersionResult {
        /// Name of the Beanstalk Application the version is associated with.
        pub application: pulumi_gestalt_rust::Output<String>,
        /// ARN assigned by AWS for this Elastic Beanstalk Application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// S3 bucket that contains the Application Version source bundle.
        pub bucket: pulumi_gestalt_rust::Output<String>,
        /// Short description of the Application Version.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// On delete, force an Application Version to be deleted when it may be in use by multiple Elastic Beanstalk Environments.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// S3 object that is the Application Version source bundle.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the this Application Version.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Pre-processes and validates the environment manifest (env.yaml ) and configuration files (*.config files in the .ebextensions folder) in the source bundle. Validating configuration files can identify issues prior to deploying the application version to an environment. You must turn processing on for application versions that you create using AWS CodeBuild or AWS CodeCommit. For application versions built from a source bundle in Amazon S3, processing is optional. It validates Elastic Beanstalk configuration files. It doesn’t validate your application’s configuration files, like proxy server or Docker configuration.
        pub process: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Key-value map of tags for the Elastic Beanstalk Application Version. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationVersionArgs,
    ) -> ApplicationVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_binding_1 = args.application.get_output(context);
        let application_binding = application_binding_1.get_inner();
        let bucket_binding_1 = args.bucket.get_output(context);
        let bucket_binding = bucket_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let force_delete_binding_1 = args.force_delete.get_output(context);
        let force_delete_binding = force_delete_binding_1.get_inner();
        let key_binding_1 = args.key.get_output(context);
        let key_binding = key_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let process_binding_1 = args.process.get_output(context);
        let process_binding = process_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/applicationVersion:ApplicationVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "application".into(),
                    value: &application_binding,
                },
                register_interface::ObjectField {
                    name: "bucket".into(),
                    value: &bucket_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "process".into(),
                    value: &process_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationVersionResult {
            application: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("application"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            bucket: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucket"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            force_delete: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            process: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("process"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
