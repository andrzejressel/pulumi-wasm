#include <cstdio>
#include <pulumi_native.h>
#include <vector>

int main()
{
	auto engine = create_engine();

	auto output = create_output(engine, "2", false);

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

	add_export(output_result, "test");

	finish(engine);
	free_engine(engine);
}