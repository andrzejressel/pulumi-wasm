#include <pulumi_gestalt.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

static char* mapper(const void* context_context, const void* context, const char* content) {

	const char* function_name = (const char*) context;

	if (strcmp(function_name, "double") == 0) {
		int i = atoi(content);
		int i2 = i * 2;

		char *buffer = malloc(23 * sizeof(char));
		if (buffer == NULL) {
			printf("Cannot allocate buffer");
			exit(2);
		}

		snprintf(buffer, 23, "%d", i2);
		return buffer;
	}
	else if (strcmp(function_name, "static") == 0) {
		return strdup("\"my_string\"");
	}

	printf("Cannot find valid function\n");
	exit(2);
}

static void generate_random_value(pulumi_context_t* ctx) {

	pulumi_output_t* output = pulumi_create_output(ctx, "16", false);

	const pulumi_object_field_t inputs[] = {
	{
		.name = "length",
		.value = output
	}
	};

	const pulumi_register_resource_request_t register_resource_request = {
		.type_ = "random:index/randomString:RandomString",
		.name = "my_name",
		.version = "4.15.1",
		.inputs = inputs,
		.inputs_len = 1,
	};

	pulumi_composite_output_t* composite_output = pulumi_register_resource(ctx, &register_resource_request);

	pulumi_output_t* output_result = pulumi_composite_output_get_field(composite_output, "result");

	pulumi_output_add_to_export(output_result, "result");
}

static void run_command(pulumi_context_t* ctx) {
	pulumi_output_t* output = pulumi_create_output(ctx, "\"whoami\"", false);

	const pulumi_object_field_t inputs[] = {
		{
			.name = "command",
			.value = output
		}
	};

	const pulumi_invoke_resource_request_t register_resource_request = {
		.token = "command:local:run",
		.version = "1.0.2",
		.inputs = inputs,
		.inputs_len = 1,
	};

	pulumi_composite_output_t* output_2 = pulumi_invoke_resource(ctx, &register_resource_request);

	pulumi_output_t* stdout_output = pulumi_composite_output_get_field(output_2, "stdout");

	pulumi_output_add_to_export(stdout_output, "whoami_stdout");
}

static void perform_operations_on_outputs(pulumi_context_t* ctx) {

	pulumi_output_t* output = pulumi_create_output(ctx, "16", false);

	pulumi_output_t* output_2 = pulumi_output_map(ctx, output, "double", &mapper);
	pulumi_output_t* output_3 = pulumi_output_map(ctx, output, "static", &mapper);
	
	const pulumi_output_t* arr[] = { output_2, output_3 };
	pulumi_output_t* output_4 = pulumi_output_combine(output, arr, 2);
	
	pulumi_output_add_to_export(output_2, "double_length");
	pulumi_output_add_to_export(output_3, "static_string");
	pulumi_output_add_to_export(output_4, "combined");
}

int main() {
	pulumi_context_t* ctx = pulumi_create_context(NULL);

	run_command(ctx);
	perform_operations_on_outputs(ctx);
	generate_random_value(ctx);

	pulumi_finish(ctx);
	pulumi_destroy_context(ctx);
}