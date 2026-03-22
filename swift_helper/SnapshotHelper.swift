@preconcurrency import SwiftUI

#if os(macOS)
import AppKit

@MainActor
func renderViewToPNG<V: View>(_ view: V, width: CGFloat, height: CGFloat) -> Data? {
    let sized = view.frame(width: width, height: height)
    let renderer = ImageRenderer(content: sized)
    renderer.scale = 1.0
    guard let image = renderer.nsImage else { return nil }
    guard let tiff = image.tiffRepresentation,
          let rep = NSBitmapImageRep(data: tiff) else { return nil }
    return rep.representation(using: .png, properties: [:])
}

@_cdecl("compare_png_bytes")
public func comparePNGBytes(
    _ a: UnsafeRawPointer, _ aLen: Int,
    _ b: UnsafeRawPointer, _ bLen: Int,
    _ tolerancePercent: Float
) -> Float {
    guard let imgA = NSImage(data: Data(bytes: a, count: aLen)),
          let imgB = NSImage(data: Data(bytes: b, count: bLen)),
          let repA = imgA.representations.first as? NSBitmapImageRep,
          let repB = imgB.representations.first as? NSBitmapImageRep else { return -1.0 }
    let w = min(repA.pixelsWide, repB.pixelsWide)
    let h = min(repA.pixelsHigh, repB.pixelsHigh)
    if w == 0 || h == 0 { return -1.0 }
    var matching = 0
    let total = w * h
    let tol = Double(tolerancePercent) / 100.0
    for y in 0..<h {
        for x in 0..<w {
            guard let ca = repA.colorAt(x: x, y: y), let cb = repB.colorAt(x: x, y: y) else { continue }
            if abs(ca.redComponent - cb.redComponent) <= tol &&
               abs(ca.greenComponent - cb.greenComponent) <= tol &&
               abs(ca.blueComponent - cb.blueComponent) <= tol &&
               abs(ca.alphaComponent - cb.alphaComponent) <= tol { matching += 1 }
        }
    }
    return Float(matching) / Float(total) * 100.0
}

#else // iOS

import UIKit

@MainActor
func renderViewToPNG<V: View>(_ view: V, width: CGFloat, height: CGFloat) -> Data? {
    let sized = view.frame(width: width, height: height)
    let renderer = ImageRenderer(content: sized)
    renderer.scale = 1.0
    guard let image = renderer.uiImage else { return nil }
    return image.pngData()
}

@_cdecl("compare_png_bytes")
public func comparePNGBytes(
    _ a: UnsafeRawPointer, _ aLen: Int,
    _ b: UnsafeRawPointer, _ bLen: Int,
    _ tolerancePercent: Float
) -> Float {
    guard let imgA = UIImage(data: Data(bytes: a, count: aLen)),
          let imgB = UIImage(data: Data(bytes: b, count: bLen)),
          let cgA = imgA.cgImage, let cgB = imgB.cgImage else { return -1.0 }
    let w = min(cgA.width, cgB.width)
    let h = min(cgA.height, cgB.height)
    if w == 0 || h == 0 { return -1.0 }
    // Simplified: compare raw pixel data
    guard let dataA = cgA.dataProvider?.data, let dataB = cgB.dataProvider?.data else { return -1.0 }
    let ptrA = CFDataGetBytePtr(dataA)
    let ptrB = CFDataGetBytePtr(dataB)
    let bpp = cgA.bitsPerPixel / 8
    let tol = Int(tolerancePercent * 2.55)
    var matching = 0
    let total = w * h
    for i in 0..<total {
        let off = i * bpp
        var match = true
        for c in 0..<min(bpp, 4) {
            if abs(Int(ptrA![off+c]) - Int(ptrB![off+c])) > tol { match = false; break }
        }
        if match { matching += 1 }
    }
    return Float(matching) / Float(total) * 100.0
}

#endif

// ── Cross-platform snapshot functions ──

@MainActor @_cdecl("snapshot_text")
public func snapshotText(
    _ utf8: UnsafePointer<UInt8>, _ len: Int,
    _ width: Float, _ height: Float,
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    let s = String(bytes: UnsafeBufferPointer(start: utf8, count: len), encoding: .utf8) ?? ""
    guard let data = renderViewToPNG(Text(s), width: CGFloat(width), height: CGFloat(height)) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: data.count, alignment: 1)
    data.copyBytes(to: buf.assumingMemoryBound(to: UInt8.self), count: data.count)
    outPtr.pointee = buf; outLen.pointee = data.count
    return true
}

@MainActor @_cdecl("snapshot_styled_text")
public func snapshotStyledText(
    _ utf8: UnsafePointer<UInt8>, _ len: Int,
    _ fontSize: Float, _ weight: Int32,
    _ r: Float, _ g: Float, _ b: Float, _ a: Float,
    _ width: Float, _ height: Float,
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    let s = String(bytes: UnsafeBufferPointer(start: utf8, count: len), encoding: .utf8) ?? ""
    var text = Text(s).font(.system(size: CGFloat(fontSize)))
    switch weight {
    case 1: text = text.bold()
    case 2: text = text.italic()
    case 3: text = text.bold().italic()
    default: break
    }
    let view = text.foregroundColor(Color(red: Double(r), green: Double(g), blue: Double(b), opacity: Double(a)))
    guard let data = renderViewToPNG(view, width: CGFloat(width), height: CGFloat(height)) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: data.count, alignment: 1)
    data.copyBytes(to: buf.assumingMemoryBound(to: UInt8.self), count: data.count)
    outPtr.pointee = buf; outLen.pointee = data.count
    return true
}

@MainActor @_cdecl("snapshot_vstack_texts")
public func snapshotVStackTexts(
    _ texts: UnsafePointer<UnsafePointer<UInt8>>,
    _ lens: UnsafePointer<Int>, _ count: Int,
    _ width: Float, _ height: Float,
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    let strings = (0..<count).map {
        String(bytes: UnsafeBufferPointer(start: texts[$0], count: lens[$0]), encoding: .utf8) ?? ""
    }
    let view = VStack(spacing: 8) { ForEach(strings.indices, id: \.self) { Text(strings[$0]) } }
    guard let data = renderViewToPNG(view, width: CGFloat(width), height: CGFloat(height)) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: data.count, alignment: 1)
    data.copyBytes(to: buf.assumingMemoryBound(to: UInt8.self), count: data.count)
    outPtr.pointee = buf; outLen.pointee = data.count
    return true
}

@MainActor @_cdecl("snapshot_view_handle")
public func snapshotViewHandle(
    _ handle: UnsafeMutableRawPointer,
    _ width: Float, _ height: Float,
    _ outPtr: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outLen: UnsafeMutablePointer<Int>
) -> Bool {
    let anyView = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as! AnyView
    guard let data = renderViewToPNG(anyView, width: CGFloat(width), height: CGFloat(height)) else { return false }
    let buf = UnsafeMutableRawPointer.allocate(byteCount: data.count, alignment: 1)
    data.copyBytes(to: buf.assumingMemoryBound(to: UInt8.self), count: data.count)
    outPtr.pointee = buf; outLen.pointee = data.count
    return true
}

@_cdecl("free_snapshot")
public func freeSnapshot(_ ptr: UnsafeMutableRawPointer, _ len: Int) { ptr.deallocate() }
