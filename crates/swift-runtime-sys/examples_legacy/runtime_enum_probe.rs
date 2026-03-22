/// Enum construction and introspection probe for Track D.3.
/// Tests: Direction (raw-representable), Shape (associated values), case extraction.
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| panic!("failed to init RuntimeFactory: {e:?}"));

    let _descriptor = factory
        .validate_runtime_contract(1)
        .unwrap_or_else(|e| panic!("runtime contract validation failed: {e:?}"));

    let contract = RuntimeContract::new(&factory);

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Direction Enum Tests (raw-representable Int32)
    println!("\n=== Direction Enum Tests (Raw-Representable) ===");

    // Test 1: Construct Direction.north (case_id=0)
    match test_direction_north(&contract) {
        Ok(true) => {
            println!("✓ Direction.north construction PASS");
            tests_passed += 1;
        }
        Ok(false) => {
            println!("✗ Direction.north construction FAIL - assertion or comparison error");
            tests_failed += 1;
        }
        Err(e) => {
            println!("✗ Direction.north construction FAIL - error: {e:?}");
            tests_failed += 1;
        }
    }

    // Test 2: Construct Direction.south (case_id=1)
    match test_direction_south(&contract) {
        Ok(true) => {
            println!("✓ Direction.south construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Direction.south construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 3: Construct Direction.east (case_id=2)
    match test_direction_east(&contract) {
        Ok(true) => {
            println!("✓ Direction.east construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Direction.east construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 4: Construct Direction.west (case_id=3)
    match test_direction_west(&contract) {
        Ok(true) => {
            println!("✓ Direction.west construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Direction.west construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 5: Invalid Direction case (case_id=99)
    match test_direction_invalid(&contract) {
        Ok(true) => {
            println!("✓ Direction invalid case rejection PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Direction invalid case rejection FAIL");
            tests_failed += 1;
        }
    }

    // Test 6: Direction round-trip (construct, extract case)
    match test_direction_roundtrip_all(&contract) {
        Ok(true) => {
            println!("✓ Direction round-trip (all cases) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Direction round-trip (all cases) FAIL");
            tests_failed += 1;
        }
    }

    // Shape Enum Tests (associated values)
    println!("\n=== Shape Enum Tests (Associated Values) ===");

    // Test 7: Construct Shape.circle
    match test_shape_circle(&contract) {
        Ok(true) => {
            println!("✓ Shape.circle construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.circle construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 8: Construct Shape.rectangle
    match test_shape_rectangle(&contract) {
        Ok(true) => {
            println!("✓ Shape.rectangle construction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.rectangle construction FAIL");
            tests_failed += 1;
        }
    }

    // Test 9: Extract case from circle
    match test_shape_circle_case(&contract) {
        Ok(true) => {
            println!("✓ Shape.circle case extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.circle case extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 10: Extract case from rectangle
    match test_shape_rect_case(&contract) {
        Ok(true) => {
            println!("✓ Shape.rectangle case extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.rectangle case extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 11: Extract radius from circle
    match test_shape_circle_radius(&contract) {
        Ok(true) => {
            println!("✓ Shape.circle radius extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.circle radius extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 12: Extract dimensions from rectangle
    match test_shape_rect_dims(&contract) {
        Ok(true) => {
            println!("✓ Shape.rectangle dims extraction PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.rectangle dims extraction FAIL");
            tests_failed += 1;
        }
    }

    // Test 13: Shape round-trip (circle)
    match test_shape_roundtrip_circle(&contract) {
        Ok(true) => {
            println!("✓ Shape.circle round-trip (construct, extract, verify) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.circle round-trip (construct, extract, verify) FAIL");
            tests_failed += 1;
        }
    }

    // Test 14: Shape round-trip (rectangle)
    match test_shape_roundtrip_rect(&contract) {
        Ok(true) => {
            println!("✓ Shape.rectangle round-trip (construct, extract, verify) PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Shape.rectangle round-trip (construct, extract, verify) FAIL");
            tests_failed += 1;
        }
    }

    // Test 15: Multiple shapes in sequence
    match test_shape_multiple_sequence(&contract) {
        Ok(true) => {
            println!("✓ Multiple shape construction sequence PASS");
            tests_passed += 1;
        }
        _ => {
            println!("✗ Multiple shape construction sequence FAIL");
            tests_failed += 1;
        }
    }

    // Cleanup and summary
    println!("\n=== Enum Parity Probe Summary ===");
    println!("Tests Passed: {}", tests_passed);
    println!("Tests Failed: {}", tests_failed);

    if tests_failed == 0 {
        println!("✓ All enum introspection tests PASSED (Track D.3 complete)");
    } else {
        panic!("✗ Enum introspection tests FAILED");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Direction Enum Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_direction_north(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let north = contract.construct_direction(0)?;
    assert_eq!(north.type_id, 9);
    let extracted = contract.direction_case(north)?;
    Ok(extracted == 0)
}

fn test_direction_south(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let south = contract.construct_direction(1)?;
    assert_eq!(south.type_id, 9);
    let extracted = contract.direction_case(south)?;
    Ok(extracted == 1)
}

fn test_direction_east(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let east = contract.construct_direction(2)?;
    assert_eq!(east.type_id, 9);
    let extracted = contract.direction_case(east)?;
    Ok(extracted == 2)
}

fn test_direction_west(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let west = contract.construct_direction(3)?;
    assert_eq!(west.type_id, 9);
    let extracted = contract.direction_case(west)?;
    Ok(extracted == 3)
}

fn test_direction_invalid(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Attempting to construct an invalid case should fail
    match contract.construct_direction(99) {
        Err(RuntimeContractError::NullConstruct { type_id: 9 }) => Ok(true),
        _ => Ok(false),
    }
}

fn test_direction_roundtrip_all(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    for case_id in 0..4 {
        let dir = contract.construct_direction(case_id)?;
        let extracted = contract.direction_case(dir)?;
        if extracted != case_id {
            return Ok(false);
        }
    }
    Ok(true)
}

// ─────────────────────────────────────────────────────────────────────────────
// Shape Enum Tests
// ─────────────────────────────────────────────────────────────────────────────

fn test_shape_circle(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let circle = contract.construct_shape_circle(5.0)?;
    assert_eq!(circle.type_id, 10);
    Ok(true)
}

fn test_shape_rectangle(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let rect = contract.construct_shape_rect(3.0, 4.0)?;
    assert_eq!(rect.type_id, 10);
    Ok(true)
}

fn test_shape_circle_case(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let circle = contract.construct_shape_circle(5.0)?;
    let case_id = contract.shape_get_case(circle)?;
    Ok(case_id == 0)
}

fn test_shape_rect_case(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let rect = contract.construct_shape_rect(3.0, 4.0)?;
    let case_id = contract.shape_get_case(rect)?;
    Ok(case_id == 1)
}

fn test_shape_circle_radius(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let circle = contract.construct_shape_circle(5.0)?;
    let radius = contract.shape_circle_radius(circle)?;
    Ok((radius - 5.0).abs() < 0.001)
}

fn test_shape_rect_dims(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let rect = contract.construct_shape_rect(3.0, 4.0)?;
    let (width, height) = contract.shape_rect_dims(rect)?;
    Ok((width - 3.0).abs() < 0.001 && (height - 4.0).abs() < 0.001)
}

fn test_shape_roundtrip_circle(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let radius = 7.5;
    let circle = contract.construct_shape_circle(radius)?;

    // Verify case
    let case_id = contract.shape_get_case(circle)?;
    if case_id != 0 {
        return Ok(false);
    }

    // Extract and verify radius
    let extracted_radius = contract.shape_circle_radius(circle)?;
    Ok((extracted_radius - radius).abs() < 0.001)
}

fn test_shape_roundtrip_rect(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    let width = 5.0;
    let height = 6.0;
    let rect = contract.construct_shape_rect(width, height)?;

    // Verify case
    let case_id = contract.shape_get_case(rect)?;
    if case_id != 1 {
        return Ok(false);
    }

    // Extract and verify dimensions
    let (ex_width, ex_height) = contract.shape_rect_dims(rect)?;
    Ok((ex_width - width).abs() < 0.001 && (ex_height - height).abs() < 0.001)
}

fn test_shape_multiple_sequence(contract: &RuntimeContract) -> Result<bool, RuntimeContractError> {
    // Construct and verify multiple shapes in sequence without cleanup
    let shapes: Vec<_> = vec![(2.0, "circle"), (3.0, "circle"), (4.0, "circle")]
        .into_iter()
        .map(|(r, _)| contract.construct_shape_circle(r))
        .collect::<Result<Vec<_>, _>>()?;

    // Verify all were constructed with correct type
    for shape in &shapes {
        if shape.type_id != 10 {
            return Ok(false);
        }
    }

    // Verify case extraction for each
    for shape in &shapes {
        let case_id = contract.shape_get_case(*shape)?;
        if case_id != 0 {
            return Ok(false);
        }
    }

    Ok(true)
}
