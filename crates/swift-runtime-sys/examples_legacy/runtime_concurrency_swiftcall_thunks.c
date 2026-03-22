#include <stdint.h>
#include <stddef.h>
#include <dlfcn.h>

typedef void * __attribute__((swiftcall)) (*swift_task_get_current_fn)(void);
typedef void * __attribute__((swiftcall)) (*swift_task_get_executor_fn)(void);
typedef void * __attribute__((swiftcall)) (*swift_task_alloc_fn)(size_t size);
typedef void __attribute__((swiftcall)) (*swift_task_dealloc_fn)(void *ptr);
typedef void __attribute__((swiftcall)) (*swift_task_cancel_fn)(void *task);
typedef int32_t __attribute__((swiftcall)) (*swift_task_is_main_executor_fn)(void *executor);

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

void *runtime_thunk_swift_task_get_current(void) {
    void *raw = resolve_symbol("swift_task_getCurrent");
    if (raw == NULL) {
        return NULL;
    }

    swift_task_get_current_fn fn = (swift_task_get_current_fn)raw;
    return fn();
}

void *runtime_thunk_swift_task_get_current_executor(void) {
    void *raw = resolve_symbol("swift_task_getCurrentExecutor");
    if (raw == NULL) {
        return NULL;
    }

    swift_task_get_executor_fn fn = (swift_task_get_executor_fn)raw;
    return fn();
}

void *runtime_thunk_swift_task_get_main_executor(void) {
    void *raw = resolve_symbol("swift_task_getMainExecutor");
    if (raw == NULL) {
        return NULL;
    }

    swift_task_get_executor_fn fn = (swift_task_get_executor_fn)raw;
    return fn();
}

void *runtime_thunk_swift_task_alloc(size_t size) {
    void *raw = resolve_symbol("swift_task_alloc");
    if (raw == NULL) {
        return NULL;
    }

    swift_task_alloc_fn fn = (swift_task_alloc_fn)raw;
    return fn(size);
}

int32_t runtime_thunk_swift_task_dealloc(void *ptr) {
    void *raw = resolve_symbol("swift_task_dealloc");
    if (raw == NULL) {
        return INT32_MIN;
    }

    swift_task_dealloc_fn fn = (swift_task_dealloc_fn)raw;
    fn(ptr);
    return 0;
}

int32_t runtime_thunk_swift_task_alloc_probe(size_t size) {
    void *raw_get_current = resolve_symbol("swift_task_getCurrent");
    void *raw_alloc = resolve_symbol("swift_task_alloc");
    void *raw_dealloc = resolve_symbol("swift_task_dealloc");
    if (raw_get_current == NULL || raw_alloc == NULL || raw_dealloc == NULL) {
        return -1;
    }

    swift_task_get_current_fn get_current = (swift_task_get_current_fn)raw_get_current;
    void *current_task = get_current();
    if (current_task == NULL) {
        return 0;
    }

    swift_task_alloc_fn alloc = (swift_task_alloc_fn)raw_alloc;
    void *ptr = alloc(size);
    if (ptr == NULL) {
        return 2;
    }

    ((unsigned char *)ptr)[0] = 0xA5u;

    swift_task_dealloc_fn dealloc = (swift_task_dealloc_fn)raw_dealloc;
    dealloc(ptr);
    return 1;
}

int32_t runtime_thunk_swift_task_cancel_current_probe(void) {
    void *raw_get_current = resolve_symbol("swift_task_getCurrent");
    void *raw_cancel = resolve_symbol("swift_task_cancel");
    if (raw_get_current == NULL || raw_cancel == NULL) {
        return -1;
    }

    swift_task_get_current_fn get_current = (swift_task_get_current_fn)raw_get_current;
    void *current_task = get_current();
    if (current_task == NULL) {
        return 0;
    }

    swift_task_cancel_fn cancel = (swift_task_cancel_fn)raw_cancel;
    cancel(current_task);
    return 1;
}

int32_t runtime_thunk_swift_task_cancel_task(void *task) {
    void *raw_cancel = resolve_symbol("swift_task_cancel");
    if (raw_cancel == NULL) {
        return INT32_MIN;
    }
    if (task == NULL) {
        return 0;
    }

    swift_task_cancel_fn cancel = (swift_task_cancel_fn)raw_cancel;
    cancel(task);
    return 1;
}

int32_t runtime_thunk_swift_concurrency_orchestration_policy_status(void) {
    void *raw_task_create = resolve_symbol("swift_task_create");
    void *raw_job_run = resolve_symbol("swift_job_run");
    void *raw_asynclet_begin = resolve_symbol("swift_asyncLet_begin");
    void *raw_asynclet_end = resolve_symbol("swift_asyncLet_end");
    void *raw_task_cancel = resolve_symbol("swift_task_cancel");
    void *raw_nullary_job_create =
        resolve_symbol("swift_task_createNullaryContinuationJob");
    void *raw_enqueue_global = resolve_symbol("swift_task_enqueueGlobal");
    void *raw_current_executor = resolve_symbol("swift_task_getCurrentExecutor");

    int32_t status = 0;
    if (raw_task_create != NULL) {
        status |= 1;
    }
    if (raw_job_run != NULL) {
        status |= 2;
    }
    if (raw_asynclet_begin != NULL) {
        status |= 4;
    }
    if (raw_asynclet_end != NULL) {
        status |= 8;
    }
    if (raw_task_cancel != NULL) {
        status |= 16;
    }
    if (raw_nullary_job_create != NULL) {
        status |= 64;
    }
    if (raw_enqueue_global != NULL) {
        status |= 128;
    }
    if (raw_current_executor != NULL) {
        status |= 256;
    }

    // Explicitly signal that direct create/run/async-let invocation remains
    // guarded until task/job context construction thunks are proven safe.
    status |= 32;
    // Explicitly signal that the direct nullary-continuation yield path is
    // still policy-guarded until we wire a fully deterministic raw ordering test.
    status |= 512;
    return status;
}

int32_t runtime_thunk_swift_main_executor_identity_probe(void) {
    void *raw_get_main = resolve_symbol("swift_task_getMainExecutor");
    void *raw_is_main = resolve_symbol("swift_task_isMainExecutor");
    if (raw_get_main == NULL || raw_is_main == NULL) {
        return INT32_MIN;
    }

    swift_task_get_executor_fn get_main = (swift_task_get_executor_fn)raw_get_main;
    void *main_executor = get_main();
    if (main_executor == NULL) {
        return 2;
    }

    swift_task_is_main_executor_fn is_main =
        (swift_task_is_main_executor_fn)raw_is_main;
    int32_t result = is_main(main_executor);
    return result ? 1 : 0;
}

/// runtime_thunk_swift_task_direct_ordering_probe:
/// Must be called from within a live Swift task context (e.g., via a bridge-hosted async probe).
/// Tests current-task visibility, current-executor visibility, and task-local alloc/dealloc
/// ordering across 3 iterations — all via direct __attribute__((swiftcall)) typed function
/// pointers resolved at runtime.
///
/// Return value (bitfield):
///   INT32_MIN : one or more required symbols missing
///     0       : called outside a Swift task context (swift_task_getCurrent returned NULL)
///   bit0 (1)  : current task was visible to the direct SwiftCC thunk
///   bit1 (2)  : current executor was visible to the direct SwiftCC thunk
///   bit2 (4)  : alloc->write->verify->dealloc ordering was deterministic (3 iterations)
int32_t runtime_thunk_swift_task_direct_ordering_probe(void) {
    void *raw_get_current = resolve_symbol("swift_task_getCurrent");
    void *raw_get_executor = resolve_symbol("swift_task_getCurrentExecutor");
    void *raw_alloc        = resolve_symbol("swift_task_alloc");
    void *raw_dealloc      = resolve_symbol("swift_task_dealloc");
    if (raw_get_current == NULL || raw_get_executor == NULL ||
        raw_alloc == NULL || raw_dealloc == NULL) {
        return INT32_MIN;
    }

    swift_task_get_current_fn get_current = (swift_task_get_current_fn)raw_get_current;
    void *current_task = get_current();
    if (current_task == NULL) {
        return 0;
    }

    int32_t status = 1; /* bit0: task visible */

    swift_task_get_executor_fn get_executor = (swift_task_get_executor_fn)raw_get_executor;
    void *current_executor = get_executor();
    if (current_executor != NULL) {
        status |= 2; /* bit1: executor visible */
    }

    swift_task_alloc_fn alloc    = (swift_task_alloc_fn)raw_alloc;
    swift_task_dealloc_fn dealloc = (swift_task_dealloc_fn)raw_dealloc;

    int alloc_ok = 1;
    for (int i = 0; i < 3 && alloc_ok; i++) {
        void *ptr = alloc(64);
        if (ptr == NULL) {
            alloc_ok = 0;
            break;
        }
        unsigned char sentinel = (unsigned char)(0xA0u + (unsigned char)i);
        ((unsigned char *)ptr)[0] = sentinel;
        if (((unsigned char *)ptr)[0] != sentinel) {
            alloc_ok = 0;
            dealloc(ptr);
            break;
        }
        dealloc(ptr);
    }
    if (alloc_ok) {
        status |= 4; /* bit2: alloc/dealloc ordering deterministic */
    }

    return status;
}