//! Charts dashboard — bar, line, area, pie charts from Rust.
//!
//! cargo run -p swift-charts --example dashboard

fn main() {
    // Load helper
    unsafe {
        use core::ffi::c_char;
        extern "C" {
            fn dlopen(path: *const c_char, mode: i32) -> *mut core::ffi::c_void;
        }
        for p in [
            c"swift_helper/libSwiftUIHelper.dylib".as_ptr(),
            c"../../swift_helper/libSwiftUIHelper.dylib".as_ptr(),
        ] {
            if !dlopen(p, 2).is_null() {
                break;
            }
        }
    }

    println!("=== Swift Charts from Rust ===\n");

    let sales = [
        ("Mon", 120.0),
        ("Tue", 250.0),
        ("Wed", 180.0),
        ("Thu", 310.0),
        ("Fri", 220.0),
        ("Sat", 400.0),
        ("Sun", 150.0),
    ];

    let bar = swift_charts::bar_chart(&sales)
        .size(350.0, 200.0)
        .x_label("Day")
        .y_label("Revenue ($)");
    println!("Bar chart: {:?}", bar.handle());

    let line = swift_charts::line_chart(&sales).size(350.0, 200.0);
    println!("Line chart: {:?}", line.handle());

    let area = swift_charts::area_chart(&sales).size(350.0, 200.0);
    println!("Area chart: {:?}", area.handle());

    let scatter = swift_charts::point_chart(&sales).size(350.0, 200.0);
    println!("Point chart: {:?}", scatter.handle());

    let market = [
        ("iOS", 28.0),
        ("Android", 44.0),
        ("Web", 18.0),
        ("Desktop", 10.0),
    ];
    let pie = swift_charts::pie_chart(&market, 0.0).size(250.0, 250.0);
    println!("Pie chart: {:?}", pie.handle());

    let donut = swift_charts::donut_chart(&market).size(250.0, 250.0);
    println!("Donut chart: {:?}", donut.handle());

    println!("\nAll charts created. Embed in SwiftUI with vstack!/hstack!.");
}
