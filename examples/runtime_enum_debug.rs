/// Debug enum probe to understand FFI failures
use swift_runtime_sys::RuntimeContract::{RuntimeContract, RuntimeContractError};
use swift_runtime_sys::RuntimeFactory::RuntimeFactory;

fn main() {
    println!("Initializing factory...");
    let factory =
        RuntimeFactory::with_thunk_library("./libRustBridge.dylib", "./libRuntimeThunks.dylib")
            .or_else(|_| RuntimeFactory::new("./libRustBridge.dylib"))
            .unwrap_or_else(|e| {
                panic!("failed to init RuntimeFactory: {e:?}");
            });

    println!("Validating contract...");
    match factory.validate_runtime_contract(1) {
        Ok(desc) => println!("Contract validated: {:?}", desc),
        Err(e) => println!("Contract validation failed: {:?}", e),
    }

    println!("Creating RuntimeContract...");
    let contract = RuntimeContract::new(&factory);

    println!("\n=== Attempting Direction.north construction ===");
    match contract.construct_direction(0) {
        Ok(dir) => {
            println!(
                "✓ Constructed Direction (type_id={}, object={:p})",
                dir.type_id, dir.object
            );

            println!("Extracting case...");
            match contract.direction_case(dir) {
                Ok(case_id) => println!("✓ Case ID: {}", case_id),
                Err(e) => println!("✗ Error extracting case: {:?}", e),
            }
        }
        Err(e) => {
            println!("✗ Failed to construct Direction: {:?}", e);
        }
    }

    println!("\n=== Attempting Shape.circle construction ===");
    match contract.construct_shape_circle(5.0) {
        Ok(shape) => {
            println!(
                "✓ Constructed Shape.circle (type_id={}, object={:p})",
                shape.type_id, shape.object
            );

            println!("Extracting case...");
            match contract.shape_get_case(shape) {
                Ok(case_id) => println!("✓ Case ID: {}", case_id),
                Err(e) => println!("✗ Error extracting case: {:?}", e),
            }

            println!("Extracting radius...");
            match contract.shape_circle_radius(shape) {
                Ok(radius) => println!("✓ Radius: {}", radius),
                Err(e) => println!("✗ Error extracting radius: {:?}", e),
            }
        }
        Err(e) => {
            println!("✗ Failed to construct Shape.circle: {:?}", e);
        }
    }
}
