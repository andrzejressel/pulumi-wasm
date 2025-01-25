#ifndef PULUMI_WASM_H
#define PULUMI_WASM_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Output Output;

typedef struct PulumiEngine PulumiEngine;

typedef struct RegisterResourceResultField {
  const char *name;
  const struct Output *output;
} RegisterResourceResultField;

typedef struct RegisterResourceResult {
  const struct RegisterResourceResultField *fields;
  uintptr_t fields_len;
} RegisterResourceResult;

typedef struct ObjectField {
  const char *name;
  const struct Output *value;
} ObjectField;

typedef struct ResultField {
  const char *name;
} ResultField;

typedef struct RegisterResourceRequest {
  const char *type_;
  const char *name;
  const char *version;
  const struct ObjectField *object;
  uintptr_t object_len;
  const struct ResultField *results;
  uintptr_t results_len;
} RegisterResourceRequest;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct PulumiEngine *create_engine(void);

void free_engine(struct PulumiEngine *t);

struct Output *create_output(struct PulumiEngine *pulumi_engine, const char *value, bool secret);

void add_export(struct PulumiEngine *pulumi_engine, const char *name, const struct Output *value);

void finish(struct PulumiEngine *pulumi_engine);

struct RegisterResourceResult register(struct PulumiEngine *pulumi_engine,
                                       const struct RegisterResourceRequest *request);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* PULUMI_WASM_H */
