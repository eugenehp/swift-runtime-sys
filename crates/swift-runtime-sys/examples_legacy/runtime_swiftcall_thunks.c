#include <stdint.h>
#include <stddef.h>
#include <dlfcn.h>

// C ABI exported thunk names used by Rust.

typedef int32_t __attribute__((swiftcall)) (*counter_increment_swift_fn)(void *obj, int32_t delta);
typedef int32_t __attribute__((swiftcall)) (*counter_increment_swift_fn_rev)(int32_t delta, void *obj);

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

int32_t runtime_thunk_counter_increment(void *obj, int32_t delta) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC9increment2bys5Int32VAG_tF");
    if (raw == NULL) {
        return INT32_MIN;
    }

    counter_increment_swift_fn fn = (counter_increment_swift_fn)raw;
    return fn(obj, delta);
}

int32_t runtime_thunk_counter_increment_rev(void *obj, int32_t delta) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC9increment2bys5Int32VAG_tF");
    if (raw == NULL) {
        return INT32_MIN;
    }

    counter_increment_swift_fn_rev fn = (counter_increment_swift_fn_rev)raw;
    return fn(delta, obj);
}

int32_t runtime_thunk_counter_increment_x20(void *obj, int32_t delta) {
    void *raw = resolve_symbol("$s10RustBridge7CounterC9increment2bys5Int32VAG_tF");
    if (raw == NULL) {
        return INT32_MIN;
    }

#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov w0, %w[arg]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [arg] "r"(delta), [fn] "r"(raw)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)obj;
    (void)delta;
    return INT32_MIN;
#endif
}
