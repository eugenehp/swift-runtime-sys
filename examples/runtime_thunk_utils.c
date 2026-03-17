#include <stdint.h>

// Calls a Swift `throws` function (Int32, Int32) -> Int32 using the swifterror
// register convention: x21 carries a swifterror slot pointer; non-null error value
// after return means the Swift function threw.
int32_t runtime_thunk_call_throws_i32_i32(
    void *fn_addr, int32_t a, int32_t b, void **error_out)
{
#if defined(__aarch64__)
    int32_t result;
    void *error_val = 0;
    void *error_reg = 0;
    void **error_slot = &error_val;
    __asm__ volatile (
        "mov w0, %w[a]\n"
        "mov w1, %w[b]\n"
        "mov x21, %[slot]\n" /* x21 points to caller-owned swifterror slot */
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        "mov %[errreg], x21\n"
        : [out] "=r"(result), [errreg] "=r"(error_reg)
        : [a] "r"(a), [b] "r"(b), [fn] "r"(fn_addr), [slot] "r"(error_slot)
        : "x0", "x1", "x21", "x30", "memory"
    );
    if (error_val == 0 && error_reg != (void *)error_slot) {
        error_val = error_reg;
    }
    *error_out = error_val;
    return result;
#else
    (void)fn_addr; (void)a; (void)b;
    *error_out = (void *)0;
    return (int32_t)(-2147483648);
#endif
}

// Calls a Swift free function (Int32, Int32) -> (Int32, Int32) (tuple return).
// In this build, the two Int32 tuple elements are packed into x0 (low/high 32-bit).
typedef struct { int32_t first; int32_t second; } I32Pair;

I32Pair runtime_thunk_call_i32_i32_to_i32_pair(void *fn_addr, int32_t a, int32_t b) {
#if defined(__aarch64__)
    uint64_t packed;
    __asm__ volatile (
        "mov w0, %w[a]\n"
        "mov w1, %w[b]\n"
        "blr %[fn]\n"
        "mov %[packed], x0\n"
        : [packed] "=r"(packed)
        : [a] "r"(a), [b] "r"(b), [fn] "r"(fn_addr)
        : "x0", "x1", "x30", "memory"
    );
    return (I32Pair){(int32_t)(packed & 0xffffffffu), (int32_t)(packed >> 32)};
#else
    (void)fn_addr; (void)a; (void)b;
    return (I32Pair){(int32_t)(-2147483648), (int32_t)(-2147483648)};
#endif
}

int32_t runtime_thunk_call_self_to_i32_x20_by_address(void *fn_addr, void *obj) {
#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [fn] "r"(fn_addr)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_call_self_to_i32_x0_by_address(void *fn_addr, void *obj) {
#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x0, %[self]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [fn] "r"(fn_addr)
        : "x0", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_call_self_to_i32_x20_x0_by_address(void *fn_addr, void *obj) {
#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov x0, %[self]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [fn] "r"(fn_addr)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_call_witness_self_x0_x1_by_address(void *fn_addr, void *obj, void *witness) {
#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x0, %[self]\n"
        "mov x1, %[wit]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [wit] "r"(witness), [fn] "r"(fn_addr)
        : "x0", "x1", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    (void)witness;
    return INT32_MIN;
#endif
}

int32_t runtime_thunk_call_witness_self_x20_x1_by_address(void *fn_addr, void *obj, void *witness) {
#if defined(__aarch64__)
    int32_t result;
    __asm__ volatile(
        "mov x20, %[self]\n"
        "mov x1, %[wit]\n"
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [self] "r"(obj), [wit] "r"(witness), [fn] "r"(fn_addr)
        : "x0", "x1", "x20", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    (void)witness;
    return INT32_MIN;
#endif
}

// Existential-indirect: x20 = &obj_slot (the TW thunk does `ldr x20,[x20]` on entry)
// This matches Swift's witness thunk calling convention for class types.
int32_t runtime_thunk_call_existential_class_to_i32_by_address(void *fn_addr, void *obj) {
#if defined(__aarch64__)
    void *obj_slot = obj;  // stack slot holding the object pointer
    int32_t result;
    __asm__ volatile(
        "mov x20, %[slot]\n"  // x20 = &obj_slot (TW thunk will deref this)
        "blr %[fn]\n"
        "mov %w[out], w0\n"
        : [out] "=r"(result)
        : [slot] "r"(&obj_slot), [fn] "r"(fn_addr)
        : "x0", "x20", "x30", "memory");
    return result;
#else
    (void)fn_addr;
    (void)obj;
    return INT32_MIN;
#endif
}
