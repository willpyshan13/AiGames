import AppKit
import Foundation

for source in CommandLine.arguments.dropFirst() {
    let input = URL(fileURLWithPath: source)
    guard let image = NSImage(contentsOf: input),
          let bitmap = NSBitmapImageRep(
            bitmapDataPlanes: nil,
            pixelsWide: 1672,
            pixelsHigh: 941,
            bitsPerSample: 8,
            samplesPerPixel: 4,
            hasAlpha: true,
            isPlanar: false,
            colorSpaceName: .deviceRGB,
            bytesPerRow: 0,
            bitsPerPixel: 0
          ),
          let context = NSGraphicsContext(bitmapImageRep: bitmap) else {
        fatalError("Cannot render \(source)")
    }
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = context
    image.draw(in: NSRect(x: 0, y: 0, width: 1672, height: 941), from: .zero, operation: .copy, fraction: 1)
    context.flushGraphics()
    NSGraphicsContext.restoreGraphicsState()
    guard let png = bitmap.representation(using: .png, properties: [:]) else {
        fatalError("Cannot encode \(source)")
    }
    let output = input.deletingPathExtension().appendingPathExtension("png")
    try! png.write(to: output)
    print(output.path)
}
