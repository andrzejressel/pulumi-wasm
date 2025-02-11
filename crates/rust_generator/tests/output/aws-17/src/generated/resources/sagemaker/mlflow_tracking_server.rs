/// Provides a SageMaker MLFlow Tracking Server resource.
///
/// ## Example Usage
///
/// ### Cognito Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = mlflow_tracking_server::create(
///         "example",
///         MlflowTrackingServerArgs::builder()
///             .artifact_store_uri("s3://${exampleAwsS3Bucket.bucket}/path")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .tracking_server_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker MLFlow Tracking Servers using the `workteam_name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/mlflowTrackingServer:MlflowTrackingServer example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mlflow_tracking_server {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MlflowTrackingServerArgs {
        /// The S3 URI for a general purpose bucket to use as the MLflow Tracking Server artifact store.
        #[builder(into)]
        pub artifact_store_uri: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of Member Definitions that contains objects that identify the workers that make up the work team.
        #[builder(into, default)]
        pub automatic_model_registration: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The version of MLflow that the tracking server uses. To see which MLflow versions are available to use, see [How it works](https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow.html#mlflow-create-tracking-server-how-it-works).
        #[builder(into, default)]
        pub mlflow_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for an IAM role in your account that the MLflow Tracking Server uses to access the artifact store in Amazon S3. The role should have AmazonS3FullAccess permissions. For more information on IAM permissions for tracking server creation, see [Set up IAM permissions for MLflow](https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow-create-tracking-server-iam.html).
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A unique string identifying the tracking server name. This string is part of the tracking server ARN.
        #[builder(into)]
        pub tracking_server_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The size of the tracking server you want to create. You can choose between "Small", "Medium", and "Large". The default MLflow Tracking Server configuration size is "Small". You can choose a size depending on the projected use of the tracking server such as the volume of data logged, number of users, and frequency of use.
        #[builder(into, default)]
        pub tracking_server_size: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The day and time of the week in Coordinated Universal Time (UTC) 24-hour standard time that weekly maintenance updates are scheduled. For example: TUE:03:30.
        #[builder(into, default)]
        pub weekly_maintenance_window_start: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct MlflowTrackingServerResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this MLFlow Tracking Server.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The S3 URI for a general purpose bucket to use as the MLflow Tracking Server artifact store.
        pub artifact_store_uri: pulumi_gestalt_rust::Output<String>,
        /// A list of Member Definitions that contains objects that identify the workers that make up the work team.
        pub automatic_model_registration: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The version of MLflow that the tracking server uses. To see which MLflow versions are available to use, see [How it works](https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow.html#mlflow-create-tracking-server-how-it-works).
        pub mlflow_version: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) for an IAM role in your account that the MLflow Tracking Server uses to access the artifact store in Amazon S3. The role should have AmazonS3FullAccess permissions. For more information on IAM permissions for tracking server creation, see [Set up IAM permissions for MLflow](https://docs.aws.amazon.com/sagemaker/latest/dg/mlflow-create-tracking-server-iam.html).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A unique string identifying the tracking server name. This string is part of the tracking server ARN.
        pub tracking_server_name: pulumi_gestalt_rust::Output<String>,
        /// The size of the tracking server you want to create. You can choose between "Small", "Medium", and "Large". The default MLflow Tracking Server configuration size is "Small". You can choose a size depending on the projected use of the tracking server such as the volume of data logged, number of users, and frequency of use.
        pub tracking_server_size: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL to connect to the MLflow user interface for the described tracking server.
        pub tracking_server_url: pulumi_gestalt_rust::Output<String>,
        /// The day and time of the week in Coordinated Universal Time (UTC) 24-hour standard time that weekly maintenance updates are scheduled. For example: TUE:03:30.
        pub weekly_maintenance_window_start: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MlflowTrackingServerArgs,
    ) -> MlflowTrackingServerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let artifact_store_uri_binding = args.artifact_store_uri.get_output(context);
        let automatic_model_registration_binding = args
            .automatic_model_registration
            .get_output(context);
        let mlflow_version_binding = args.mlflow_version.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tracking_server_name_binding = args.tracking_server_name.get_output(context);
        let tracking_server_size_binding = args.tracking_server_size.get_output(context);
        let weekly_maintenance_window_start_binding = args
            .weekly_maintenance_window_start
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/mlflowTrackingServer:MlflowTrackingServer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "artifactStoreUri".into(),
                    value: &artifact_store_uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "automaticModelRegistration".into(),
                    value: &automatic_model_registration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mlflowVersion".into(),
                    value: &mlflow_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackingServerName".into(),
                    value: &tracking_server_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trackingServerSize".into(),
                    value: &tracking_server_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weeklyMaintenanceWindowStart".into(),
                    value: &weekly_maintenance_window_start_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MlflowTrackingServerResult {
            arn: o.get_field("arn"),
            artifact_store_uri: o.get_field("artifactStoreUri"),
            automatic_model_registration: o.get_field("automaticModelRegistration"),
            mlflow_version: o.get_field("mlflowVersion"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tracking_server_name: o.get_field("trackingServerName"),
            tracking_server_size: o.get_field("trackingServerSize"),
            tracking_server_url: o.get_field("trackingServerUrl"),
            weekly_maintenance_window_start: o.get_field("weeklyMaintenanceWindowStart"),
        }
    }
}
