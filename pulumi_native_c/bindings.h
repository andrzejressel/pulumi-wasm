#ifndef PULUMI_WASM_H
#define PULUMI_WASM_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CustomRegisterOutputId CustomRegisterOutputId;

typedef struct Output Output;

typedef struct PulumiEngine PulumiEngine;

typedef struct ObjectField {
  const char *name;
  const struct Output *value;
} ObjectField;

typedef struct RegisterResourceRequest {
  const char *type_;
  const char *name;
  const char *version;
  const struct ObjectField *object;
  uintptr_t object_len;
} RegisterResourceRequest;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct PulumiEngine *create_engine(void);

void free_engine(struct PulumiEngine *t);

struct Output *create_output(struct PulumiEngine *pulumi_engine, const char *value, bool secret);

void add_export(struct PulumiEngine *pulumi_engine, const char *name, const struct Output *value);

void finish(struct PulumiEngine *pulumi_engine);

struct Output *pulumi_get_output(struct CustomRegisterOutputId *custom_register_output_id,
                                 const char *field_name);

struct CustomRegisterOutputId *pulumi_register_resource(struct PulumiEngine *pulumi_engine,
                                                        const struct RegisterResourceRequest *request);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* PULUMI_WASM_H */
