#if canImport(Charts)
import Charts
import SwiftUI

// ═══════════════════════════════════════════════════════════════════════════
// Data point — Rust sends arrays of (label, value) pairs
// ═══════════════════════════════════════════════════════════════════════════

struct ChartDataPoint: Identifiable {
    let id: Int
    let label: String
    let value: Double
    let series: String
}

func parseDataPoints(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>,
    _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>,
    _ count: Int,
    _ seriesPtr: UnsafePointer<UInt8>?,
    _ seriesLen: Int
) -> [ChartDataPoint] {
    let series = seriesPtr.flatMap { ptr in
        seriesLen > 0 ? String(bytes: UnsafeBufferPointer(start: ptr, count: seriesLen), encoding: .utf8) : nil
    } ?? ""
    return (0..<count).map { i in
        let label = String(bytes: UnsafeBufferPointer(start: labelPtrs[i], count: labelLens[i]), encoding: .utf8) ?? ""
        return ChartDataPoint(id: i, label: label, value: values[i], series: series)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Bar Chart
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_bar")
public func chartsBar(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>, _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>, _ count: Int
) -> ViewHandle {
    let data = parseDataPoints(labelPtrs, labelLens, values, count, nil, 0)
    return boxView(
        Chart(data) { d in
            BarMark(x: .value("Label", d.label), y: .value("Value", d.value))
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Line Chart
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_line")
public func chartsLine(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>, _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>, _ count: Int
) -> ViewHandle {
    let data = parseDataPoints(labelPtrs, labelLens, values, count, nil, 0)
    return boxView(
        Chart(data) { d in
            LineMark(x: .value("Label", d.label), y: .value("Value", d.value))
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Area Chart
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_area")
public func chartsArea(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>, _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>, _ count: Int
) -> ViewHandle {
    let data = parseDataPoints(labelPtrs, labelLens, values, count, nil, 0)
    return boxView(
        Chart(data) { d in
            AreaMark(x: .value("Label", d.label), y: .value("Value", d.value))
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Point Chart (scatter)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_point")
public func chartsPoint(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>, _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>, _ count: Int
) -> ViewHandle {
    let data = parseDataPoints(labelPtrs, labelLens, values, count, nil, 0)
    return boxView(
        Chart(data) { d in
            PointMark(x: .value("Label", d.label), y: .value("Value", d.value))
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Pie / Donut Chart (SectorMark)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_pie")
public func chartsPie(
    _ labelPtrs: UnsafePointer<UnsafePointer<UInt8>>, _ labelLens: UnsafePointer<Int>,
    _ values: UnsafePointer<Double>, _ count: Int,
    _ innerRadius: Float
) -> ViewHandle {
    let data = parseDataPoints(labelPtrs, labelLens, values, count, nil, 0)
    return boxView(
        Chart(data) { d in
            SectorMark(angle: .value("Value", d.value), innerRadius: .ratio(Double(innerRadius)))
                .foregroundStyle(by: .value("Label", d.label))
        }
    )
}

// ═══════════════════════════════════════════════════════════════════════════
// Rule (horizontal/vertical reference line)
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_rule_h")
public func chartsRuleH(_ y: Double) -> ViewHandle {
    boxView(Chart { RuleMark(y: .value("Y", y)).foregroundStyle(.red) })
}

// ═══════════════════════════════════════════════════════════════════════════
// Chart modifiers
// ═══════════════════════════════════════════════════════════════════════════

@_cdecl("charts_frame")
public func chartsFrame(_ h: ViewHandle, _ w: Float, _ ht: Float) -> ViewHandle {
    boxView(unboxView(h).frame(width: CGFloat(w), height: CGFloat(ht)))
}

@_cdecl("charts_x_label")
public func chartsXLabel(_ h: ViewHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return boxView(unboxView(h).chartXAxisLabel(label))
}

@_cdecl("charts_y_label")
public func chartsYLabel(_ h: ViewHandle, _ ptr: UnsafePointer<UInt8>, _ len: Int) -> ViewHandle {
    let label = String(bytes: UnsafeBufferPointer(start: ptr, count: len), encoding: .utf8) ?? ""
    return boxView(unboxView(h).chartYAxisLabel(label))
}

#endif
