#include <cstdio>
#include <pulumi_gestalt.h>
#include <vector>
#include <string>
#include <cstring>

static char* mapper(const void*, const void* context, const char* content) {

	const char* function_name = static_cast<const char*>(context);

	if (strcmp(function_name, "double") == 0) {

		auto i = atoi(content);
		auto i2 = i * 2;
		auto result = std::to_string(i2);

		char* cstr = new char[result.size() + 1];
		strcpy(cstr, result.c_str());
		return cstr;
	}
	else if (strcmp(function_name, "static") == 0) {
		return strdup("\"my_string\"");
	}

	printf("Cannot find valid function\n");
	exit(2);
}

static void generate_random_value(pulumi_context_t* ctx) {

	auto output = pulumi_create_output(ctx, "16", false);

	std::vector<pulumi_object_field_t> inputs = {
		{"length", output}
	};

	auto const register_resource_request = pulumi_register_resource_request_t{
		.type_ = "random:index/randomString:RandomString",
		.name = "my_name",
		.version = "4.15.1",
		.inputs = inputs.data(),
		.inputs_len = inputs.size(),
	};

	auto output_2 = pulumi_register_resource(ctx, &register_resource_request);

	auto output_result = pulumi_composite_output_get_field(output_2, "result");

	auto double_length = pulumi_output_map(ctx, output, "double", &mapper);
	auto static_string = pulumi_output_map(ctx, output, "static", &mapper);

	pulumi_output_add_to_export(output_result, "result");
	pulumi_output_add_to_export(double_length, "double_length");
	pulumi_output_add_to_export(static_string, "static_string");

}


static void run_command(pulumi_context_t* ctx) {
	auto output = pulumi_create_output(ctx, "\"whoami\"", false);

	std::vector<pulumi_object_field_t> inputs = {
		{"command", output}
	};

	auto const register_resource_request = pulumi_invoke_resource_request_t{
		.token = "command:local:run",
		.version = "1.0.2",
		.inputs = inputs.data(),
		.inputs_len = inputs.size(),
	};

	auto output_2 = pulumi_invoke_resource(ctx, &register_resource_request);

	auto stdout_output = pulumi_composite_output_get_field(output_2, "stdout");

	pulumi_output_add_to_export(stdout_output, "whoami_stdout");
}

int main()
{
	auto ctx = pulumi_create_context(nullptr);

	generate_random_value(ctx);
	run_command(ctx);

	pulumi_finish(ctx);
	pulumi_destroy_context(ctx);
}