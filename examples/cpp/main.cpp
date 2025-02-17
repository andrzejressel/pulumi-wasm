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

int main()
{
	auto engine = create_engine(nullptr);

	auto output = create_output(engine, "16", false);

	std::vector<pulumi_object_field_t> inputs = {
		{"length", output}
	};

	auto const register_resource_request = pulumi_register_resource_request_t {
		.type_ = "random:index/randomString:RandomString",
		.name = "my_name",
		.version = "4.15.1",
		.object = inputs.data(),
		.object_len = inputs.size(),
	};

	auto output_2 = pulumi_register_resource(engine, &register_resource_request);

	auto output_result = pulumi_get_output(output_2, "result");

	auto double_length = pulumi_map(engine, output, "double", &mapper);
	auto static_string = pulumi_map(engine, output, "static", &mapper);

	add_export(output_result, "result");
	add_export(double_length, "double_length");
	add_export(static_string, "static_string");

	finish(engine);
	free_engine(engine);
}