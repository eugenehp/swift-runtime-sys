use swift_runtime_sys::RuntimeContract::{
    CompilerFeatureStatus, ContractArgValue, ContractOwnership, ContractResultValue,
    RuntimeContract,
};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

const TYPE_PERSON: i32 = 1;
const TYPE_COUNTER: i32 = 2;
const PERSON_GET_ID: i32 = 1;
const PERSON_GET_AGE: i32 = 2;
const COUNTER_INCREMENT: i32 = 1;
const COUNTER_CURRENT: i32 = 2;
const COUNTER_RESET: i32 = 3;
const COUNTER_CURRENT_VIA_PROTOCOL: i32 = 4;
const METADATA_PERSON: i32 = 1;
const METADATA_COUNTER: i32 = 2;
const METADATA_GENERIC_BOX_I32: i32 = 1001;
const PROTOCOL_COUNTERLIKE: i32 = 1;
const PROTOCOL_METHOD_CURRENT: i32 = 1;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));
    let contract = RuntimeContract::new(&factory);
    let json = contract
        .contract_json()
        .unwrap_or_else(|e| panic!("failed to load contract JSON: {e:?}"));
    let descriptor_json = contract
        .descriptor()
        .unwrap_or_else(|e| panic!("failed to parse contract JSON: {e:?}"));
    let boundary = contract
        .cooperation_boundary()
        .unwrap_or_else(|e| panic!("failed to load cooperation boundary: {e:?}"));
    let compiler_features = contract
        .compiler_feature_capabilities()
        .unwrap_or_else(|e| panic!("failed to negotiate compiler feature capabilities: {e:?}"));

    assert_eq!(
        json, descriptor.json,
        "contract JSON mismatch between loader paths"
    );
    assert_eq!(descriptor_json.contract_version, 1);
    assert_eq!(descriptor_json.bridge, "RustBridge");
    assert!(!boundary.swift_side.is_empty());
    assert!(!boundary.rust_side.is_empty());
    assert!(!boundary.research_only.is_empty());
    assert_eq!(
        compiler_features.resilient_dispatch.status,
        CompilerFeatureStatus::Supported
    );
    assert_eq!(
        compiler_features.generic_metadata_registry.status,
        CompilerFeatureStatus::Supported
    );
    assert_eq!(
        compiler_features.protocol_witness_registry.status,
        CompilerFeatureStatus::Supported
    );
    assert_eq!(
        compiler_features.raw_runtime_research_mode.status,
        CompilerFeatureStatus::Fallback
    );
    assert_eq!(
        compiler_features.resilient_dispatch.provider,
        "swift_bridge"
    );
    assert!(compiler_features
        .generic_metadata_registry
        .reason
        .contains("deterministic metadata lookup"));

    let person_meta = contract
        .lookup_metadata(METADATA_PERSON)
        .unwrap_or_else(|e| panic!("failed to lookup Person metadata: {e:?}"));
    let counter_meta = contract
        .lookup_metadata(METADATA_COUNTER)
        .unwrap_or_else(|e| panic!("failed to lookup Counter metadata: {e:?}"));
    let generic_box_meta = contract
        .lookup_metadata(METADATA_GENERIC_BOX_I32)
        .unwrap_or_else(|e| panic!("failed to lookup ContractGenericBox<Int32> metadata: {e:?}"));

    assert!(!person_meta.is_null());
    assert!(!counter_meta.is_null());
    assert!(!generic_box_meta.is_null());

    let person = match contract
        .construct_boxed(
            TYPE_PERSON,
            &[ContractArgValue::I32(7), ContractArgValue::I32(41)],
        )
        .unwrap_or_else(|e| panic!("failed to construct Person: {e:?}"))
    {
        ContractResultValue::OwnedObject(object) => object,
        other => panic!("unexpected boxed Person result: {other:?}"),
    };
    assert!(matches!(
        person.ownership(),
        ContractOwnership::SwiftRetained
    ));
    let person_id = match contract
        .invoke_i32_boxed(person.as_object(), PERSON_GET_ID, &[])
        .unwrap_or_else(|e| panic!("failed to invoke Person.get_id: {e:?}"))
    {
        ContractResultValue::I32(value) => value,
        other => panic!("unexpected boxed Person.get_id result: {other:?}"),
    };
    let person_age = match contract
        .invoke_i32_boxed(person.as_object(), PERSON_GET_AGE, &[])
        .unwrap_or_else(|e| panic!("failed to invoke Person.get_age: {e:?}"))
    {
        ContractResultValue::I32(value) => value,
        other => panic!("unexpected boxed Person.get_age result: {other:?}"),
    };

    let counter = match contract
        .construct_boxed(TYPE_COUNTER, &[ContractArgValue::I32(10)])
        .unwrap_or_else(|e| panic!("failed to construct Counter: {e:?}"))
    {
        ContractResultValue::OwnedObject(object) => object,
        other => panic!("unexpected boxed Counter result: {other:?}"),
    };
    assert!(matches!(
        counter.ownership(),
        ContractOwnership::SwiftRetained
    ));
    let incremented = match contract
        .invoke_i32_boxed(
            counter.as_object(),
            COUNTER_INCREMENT,
            &[ContractArgValue::I32(5)],
        )
        .unwrap_or_else(|e| panic!("failed to invoke Counter.increment: {e:?}"))
    {
        ContractResultValue::I32(value) => value,
        other => panic!("unexpected boxed Counter.increment result: {other:?}"),
    };
    let current = match contract
        .invoke_i32_boxed(counter.as_object(), COUNTER_CURRENT, &[])
        .unwrap_or_else(|e| panic!("failed to invoke Counter.current: {e:?}"))
    {
        ContractResultValue::I32(value) => value,
        other => panic!("unexpected boxed Counter.current result: {other:?}"),
    };
    match contract
        .invoke_void_boxed(
            counter.as_object(),
            COUNTER_RESET,
            &[ContractArgValue::I32(3)],
        )
        .unwrap_or_else(|e| panic!("failed to invoke Counter.reset: {e:?}"))
    {
        ContractResultValue::Void => {}
        other => panic!("unexpected boxed Counter.reset result: {other:?}"),
    }
    let via_protocol = match contract
        .invoke_i32_boxed(counter.as_object(), COUNTER_CURRENT_VIA_PROTOCOL, &[])
        .unwrap_or_else(|e| panic!("failed to invoke CounterLike.current: {e:?}"))
    {
        ContractResultValue::I32(value) => value,
        other => panic!("unexpected boxed CounterLike.current result: {other:?}"),
    };
    let has_counterlike = contract
        .protocol_has_conformance(TYPE_COUNTER, PROTOCOL_COUNTERLIKE)
        .unwrap_or_else(|e| panic!("failed to query CounterLike conformance: {e:?}"));
    assert!(has_counterlike);
    let via_registry = contract
        .protocol_invoke_i32(
            counter.as_object(),
            PROTOCOL_COUNTERLIKE,
            PROTOCOL_METHOD_CURRENT,
        )
        .unwrap_or_else(|e| panic!("failed to invoke protocol registry current(): {e:?}"));
    assert_eq!(via_registry, via_protocol);

    println!(
        "contract dispatch => person_id={} person_age={} counter_after_increment={} counter_current={} counter_via_protocol={} protocol_registry_current={} metadata_registry=true protocol_registry=true normalized=true resilient_dispatch={:?} generic_metadata={:?} protocol_registry={:?}",
        person_id,
        person_age,
        incremented,
        current,
        via_protocol,
        via_registry,
        compiler_features.resilient_dispatch.status,
        compiler_features.generic_metadata_registry.status,
        compiler_features.protocol_witness_registry.status
    );
}
