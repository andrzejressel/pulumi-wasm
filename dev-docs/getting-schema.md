Proper way of getting package schema: https://github.com/pulumi/pulumi/blob/2366960ae545d31c52f1e9a6b8b35184511c3b84/pkg/cmd/pulumi/package_extract_schema.go#L28

Basically:
1. Download executable
2. Use GRPC request GetSchema