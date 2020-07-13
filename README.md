# EXPERIMENTAL
This is a super experimental project.  Don't expect it to be stable.

## Summary
This project aims to provide a **renderer** that is:
- Simple - no need to learn the intricacies of graphics programming
- Extensible - integrate your own custom *whatever* if you want
- Fast - it's written in Rust, what's the point otherwise

![wip](https://github.com/krooq/maple/blob/develop/src/images/wip.png)

[Maple](https://en.wikipedia.org/wiki/Acer_palmatum) is just a placeholder name, still looking for inspiration.

## Motivation
Graphics programming is a totally different domain to web or systems programming.
It's quite a technically complicated job to draw things to the screen, most programmers do not have the knowledge or time to do this.
But graphics are essential to any user facing program, even that terminal application printing your program's sysout has some complicated graphics work behind it.

At a sort of high level, "usable" graphics programming can be split into several distinct tasks/abstractions
(I straight up stole these ideas from the amazingly talented Lin Clark and adapted them for my own purposes, see this great article on the Firefox's rendering engine https://hacks.mozilla.org/2017/10/the-whole-web-at-maximum-fps-how-webrender-gets-rid-of-jank/)

CPU
|           |                                                                                |
|-----------|--------------------------------------------------------------------------------|
| Model     | build a data format containing all the graphic objects                         |
| Parse     | turn that model into a usable data structure, usually a tree                   |
| Style     | link some colour and image data to the graphic objects                         |
| Layout    | apply some position and bounds rules to the graphic objects (this is optional) |
| Tesselate | turn those graphic objects into triangles                                      |

GPU
|           |                                                                                       |
|-----------|---------------------------------------------------------------------------------------|
| Paint     | turn the triangles into colored pixels, aka Rasterize                                 |
| Composite | smush together multiple "layers" that may have been created in Paint e.g. for opacity |
| Render    | actually put stuff on the screen! aka Present                                         |

So *"rendering"* (or at least my definition of it) is actually just the GPU part. 
This is all this project aims to provide but I'll also throw in some batteries to get you going.

NOTE on [Software rendering](https://en.wikipedia.org/wiki/Software_rendering):\
Modern video games, video editing and digital art require the extra performance of a dedicated GPU but rendering doesn't actually require a dedicated GPU. In fact most everyday programs such as browsers, at least in 2020, use the integrated graphics in the CPU to reduce power consumption.
This library does not aim to provide actual *"software rendering"* i.e. using the regular processing loop on a CPU although you should be able to use the integrated graphics on your CPU, this will be exposed by [wgpu-rs](https://github.com/gfx-rs/wgpu-rs) through an adapter. Although, afaik, the work in this area is experimental so your options may be limited for now.

# What I mean by *"cross platform"*?
I would like to target all the platforms that wgpu targets however some platforms are not yet mature enough in the Rust ecosystem.

The following should *just work*™:
- Windows
- Linux
- macOS

Would Love to target these platforms when the Rust ecosystem is more mature:
- WebAssembly
- Android
- iOS

## Roadmap
#### Pre-Alpha
- [x] Basic runtime (window + events)
- [x] Render colored triangle in a window
- [x] Add a mesh to scene
- [x] Delete a mesh from scene
- [ ] Remove hardcoded "examples" from library code
- [ ] Public (experimental) API
- [ ] Example images
- [ ] Code examples

#### Alpha
- [ ] Render textures on a mesh
- [ ] Render monospaced fonts

#### Beta
- [ ] Evaluate performance

#### 1.0+
- [ ] yeah right..  we will see


## Crates
There is no plan to provide multiple backends, although for some functions it should be possible to create your own and plug it in.

The crates used for builtin functions are:
- winit for windowing/event loop
- wgpu for graphics

The aim is to stay up to date with wgpu releases and sometime even up to date with the wgpu-rs main branch.
Check the [Cargo.toml](https://github.com/krooq/maple/blob/develop/Cargo.toml) for details.

## Attribution
The following projects are amazing works of art and none of the work here would be possible without them.
I want to give a massive thank you to their authors and contributors. Please go and check them out!
- [wgpu-rs](https://github.com/gfx-rs/wgpu-rs)
- [winit](https://github.com/rust-windowing/winit)
- [learn wgpu](https://sotrh.github.io/learn-wgpu/)
- [iced](https://github.com/hecrj/iced)

## License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.