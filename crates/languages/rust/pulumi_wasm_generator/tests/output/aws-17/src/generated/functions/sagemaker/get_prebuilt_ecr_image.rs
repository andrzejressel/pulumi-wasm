pub mod get_prebuilt_ecr_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrebuiltEcrImageArgs {
        /// DNS suffix to use in the registry path. If not specified, the AWS provider sets it to the DNS suffix for the current region.
        #[builder(into, default)]
        pub dns_suffix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Image tag for the Docker image. If not specified, the AWS provider sets the value to `1`, which for many repositories indicates the latest version. Some repositories, such as XGBoost, do not support `1` or `latest` and specific version must be used.
        #[builder(into, default)]
        pub image_tag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Region to use in the registry path. If not specified, the AWS provider sets it to the current region.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the repository, which is generally the algorithm or library. Values include `autogluon-inference`, `autogluon-training`, `blazingtext`, `djl-inference`, `factorization-machines`, `forecasting-deepar`, `huggingface-pytorch-inference`, `huggingface-pytorch-inference-neuron`, `huggingface-pytorch-inference-neuronx`, `huggingface-pytorch-tgi-inference`, `huggingface-pytorch-training`, `huggingface-pytorch-training-neuronx`, `huggingface-pytorch-trcomp-training`, `huggingface-tensorflow-inference`, `huggingface-tensorflow-training`, `huggingface-tensorflow-trcomp-training`, `image-classification`, `image-classification-neo`, `ipinsights`, `kmeans`, `knn`, `lda`, `linear-learner`, `mxnet-inference`, `mxnet-inference-eia`, `mxnet-training`, `ntm`, `object-detection`, `object2vec`, `pca`, `pytorch-inference`, `pytorch-inference-eia`, `pytorch-inference-graviton`, `pytorch-inference-neuronx`, `pytorch-training`, `pytorch-training-neuronx`, `pytorch-trcomp-training`, `randomcutforest`, `sagemaker-base-python`, `sagemaker-chainer`, `sagemaker-clarify-processing`, `sagemaker-data-wrangler-container`, `sagemaker-debugger-rules`, `sagemaker-geospatial-v1-0`, `sagemaker-inference-mxnet`, `sagemaker-inference-pytorch`, `sagemaker-inference-tensorflow`, `sagemaker-model-monitor-analyzer`, `sagemaker-mxnet`, `sagemaker-mxnet-eia`, `sagemaker-mxnet-serving`, `sagemaker-mxnet-serving-eia`, `sagemaker-neo-mxnet`, `sagemaker-neo-pytorch`, `sagemaker-neo-tensorflow`, `sagemaker-pytorch`, `sagemaker-rl-coach-container`, `sagemaker-rl-mxnet`, `sagemaker-rl-ray-container`, `sagemaker-rl-tensorflow`, `sagemaker-rl-vw-container`, `sagemaker-scikit-learn`, `sagemaker-spark-processing`, `sagemaker-sparkml-serving`, `sagemaker-tensorflow`, `sagemaker-tensorflow-eia`, `sagemaker-tensorflow-scriptmode`, `sagemaker-tensorflow-serving`, `sagemaker-tensorflow-serving-eia`, `sagemaker-tritonserver`, `sagemaker-xgboost`, `semantic-segmentation`, `seq2seq`, `stabilityai-pytorch-inference`, `tei`, `tei-cpu`, `tensorflow-inference`, `tensorflow-inference-eia`, `tensorflow-inference-graviton`, `tensorflow-training`, and `xgboost-neo`.
        #[builder(into)]
        pub repository_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPrebuiltEcrImageResult {
        pub dns_suffix: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub image_tag: pulumi_wasm_rust::Output<Option<String>>,
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Account ID containing the image. For example, `469771592824`.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Docker image URL. For example, `341280168497.dkr.ecr.ca-central-1.amazonaws.com/sagemaker-sparkml-serving:2.4`.
        pub registry_path: pulumi_wasm_rust::Output<String>,
        pub repository_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPrebuiltEcrImageArgs,
    ) -> GetPrebuiltEcrImageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dns_suffix_binding = args.dns_suffix.get_output(context).get_inner();
        let image_tag_binding = args.image_tag.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let repository_name_binding = args
            .repository_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sagemaker/getPrebuiltEcrImage:getPrebuiltEcrImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dnsSuffix".into(),
                    value: &dns_suffix_binding,
                },
                register_interface::ObjectField {
                    name: "imageTag".into(),
                    value: &image_tag_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "repositoryName".into(),
                    value: &repository_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPrebuiltEcrImageResult {
            dns_suffix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsSuffix"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            image_tag: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("imageTag"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            registry_path: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryPath"),
            ),
            repository_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("repositoryName"),
            ),
        }
    }
}
