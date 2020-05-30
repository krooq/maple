## What Maple is?

Maple as a project aims to achieve 3 primary goals:
1. Provide simple "cross platform" *rendering* capabilites
2. Provide a simple *UI toolkit* for trivial UI use cases
3. Provide a simple *UI design app* for trivial UI use cases

## What Maple is **not**?

Maple does not aim to do any of the following:
1. Provide complex UI widgets
2. Perform text shaping i.e. left to right, one-to-one, monospaced fonts only
3. Perform complex layouting

## Roadmap
1. Renderer
2. UI toolkit

## Principles
The design of Maple is driven by high level principles rather than rules or conventions.

These principles are:
1. Simplicity
2. Practicality
3. Empowerment

From the above principles Maple derives the following ethos:
- Easy to learn
- Easy to use
- Runs as fast as possible
- Doesn't get in your way
- Interoperable
- Extensible

## What is *"cross platform"*?
Maple aims to target (in this order):
- Web Assembly
- Windows
- macOS
- Linux

Mobile support will likely come through WASM but is a massive TBD right now.

## What is *"rendering"*?
Graphics programming is a totally differnt domain to web or systems programming.
It's a technically complicated job to draw things to the screen, most programmers do not have the knowledge or time to do this.
But graphics are essential to any user facing program, even that terminal application printing sysout has some complicated graphics work behind it.

At a sort of high level, "usable" graphics programming can be split into several distinct tasks/abstractions
(I straight up stole these ideas from the amazingly talented Lin Clark and adapted them for my own purposes, see this great article on the Firefox's rendering engine https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)

CPU
1. Model (some data format containing all the graphic objects)
2. Parse (turn that model into a usable data structure, usually a tree)
3. Style (link some colour and image data to the graphic objects)
4. Layout (apply some position and bounds rules to the graphic objects (this is optional))
5. Tesselate (turn those graphic objects into triangles)

GPU
1. Paint (turns the triangles into colored pixels)
2. Composite (multiple layers may be provided at a time e.g. for opacity, composite smushes those together)
3. Render (actually draw!)

So *"rendering"* is actually just the GPU part. This is what Maple provides.

Modern video games, video editing and digital art require the extra performance of a dedicated GPU but rendering doesn't actually require a dedicated GPU. In fact most everyday programs such as browsers, at least in 2020, use the integrated graphics in the CPU to reduce power consumption. However this trend is changing with proliferation of 4k and 8k displays.