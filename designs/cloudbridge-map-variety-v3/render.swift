import AppKit
import Foundation

let args = CommandLine.arguments
let source = URL(fileURLWithPath: args[1])
let destination = URL(fileURLWithPath: args[2])
let width = Int(args[3])!
let height = Int(args[4])!
guard let artwork = NSImage(contentsOf: source),
      let bitmap = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: width,
          pixelsHigh: height, bitsPerSample: 8, samplesPerPixel: 4,
          hasAlpha: true, isPlanar: false, colorSpaceName: .deviceRGB,
          bytesPerRow: 0, bitsPerPixel: 0),
      let context = NSGraphicsContext(bitmapImageRep: bitmap) else {
    fatalError("Cannot load vector artwork")
}
NSGraphicsContext.saveGraphicsState()
NSGraphicsContext.current = context
let canvas = NSRect(x: 0, y: 0, width: width, height: height)
NSColor.white.setFill()
canvas.fill()
artwork.draw(in: canvas, from: NSRect(origin: .zero, size: artwork.size),
             operation: .sourceOver, fraction: 1)
NSGraphicsContext.restoreGraphicsState()
try bitmap.representation(using: .png, properties: [:])!.write(to: destination)
print(destination.path)
