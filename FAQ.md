# FAQ
Your questions, answered here, for all to learn!

# What is *"cross platform"*?
Maple aims to target (in this order):
- Windows
- Linux
- macOS

Maple would love to target these platforms when the Rust ecosystem is more mature 
- WebAssembly
- Android
- iOS

# What is *"rendering"*?
Graphics programming is a totally different domain to web or systems programming.
It's a technically complicated job to draw things to the screen, most programmers do not have the knowledge or time to do this.
But graphics are essential to any user facing program, even that terminal application printing your program's sysout has some complicated graphics work behind it.

At a sort of high level, "usable" graphics programming can be split into several distinct tasks/abstractions
(I straight up stole these ideas from the amazingly talented Lin Clark and adapted them for my own purposes, see this great article on the Firefox's rendering engine https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)

CPU
1. Model (some data format containing all the graphic objects)
2. Parse (turn that model into a usable data structure, usually a tree)
3. Style (link some colour and image data to the graphic objects)
4. Layout (apply some position and bounds rules to the graphic objects (this is optional))
5. Tesselate (turn those graphic objects into triangles)

GPU
1. Paint (turns the triangles into colored pixels, aka Rasterize)
2. Composite (multiple "layers" may be created in Paint e.g. for opacity, composite smushes those together)
3. Render (actually put stuff on the screen! aka Present)

So *"rendering"* is actually just the GPU part. This is what Maple provides.

Modern video games, video editing and digital art require the extra performance of a dedicated GPU but rendering doesn't actually require a dedicated GPU. In fact most everyday programs such as browsers, at least in 2020, use the integrated graphics in the CPU to reduce power consumption. However this trend is changing with proliferation of 4k and 8k displays.