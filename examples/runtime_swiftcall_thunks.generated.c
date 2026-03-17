#include <stdint.h>
#include <stddef.h>
#include <dlfcn.h>

static void *resolve_symbol(const char *name) {
    void *sym = dlsym(RTLD_DEFAULT, name);
    if (sym != NULL) {
        return sym;
    }

    char prefixed[512] = {0};
    prefixed[0] = '_';
    for (int i = 0; i < 510 && name[i] != '\0'; i++) {
        prefixed[i + 1] = name[i];
        prefixed[i + 2] = '\0';
    }
    return dlsym(RTLD_DEFAULT, prefixed);
}

int32_t runtime_thunk_counter_increment_x20(void *obj, int32_t arg0) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC9increment2bys5Int32VAG_tF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov w0, %w[arg0]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [arg0] "r"(arg0), [fn] "r"(raw)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)obj;
    (void)arg0;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_counter_current_x20(void *obj) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC7currents5Int32VyF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [fn] "r"(raw)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)obj;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_counter_reset_x20(void *obj, int32_t arg0) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC5reset2toys5Int32V_tF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov w0, %w[arg0]\n"
        "blr %[fn]\n"
        :
        : [self] "r"(obj), [arg0] "r"(arg0), [fn] "r"(raw)
        : "x0", "x20", "x30", "memory");
    return 0;
#else
    (void)obj;
    (void)arg0;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_counter_add_pair_x20(void *obj, int32_t arg0, int32_t arg1) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC7addPairys5Int32VAF_AFtF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov w0, %w[arg0]\n"
        "mov w1, %w[arg1]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [arg0] "r"(arg0), [arg1] "r"(arg1), [fn] "r"(raw)
        : "x0", "x1", "x20", "x30", "memory");
    return result;
#else
    (void)obj;
    (void)arg0;
    (void)arg1;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_counter_clear_x20(void *obj) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC5clearyyF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    __asm__ volatile(
        "mov x20, %[self]\n"
        "blr %[fn]\n"
        :
        : [self] "r"(obj), [fn] "r"(raw)
        : "x0", "x20", "x30", "memory");
    return 0;
#else
    (void)obj;
    return INT32_MIN;
#endif
}

