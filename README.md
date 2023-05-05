# Gravy Galaxy

> 3D Gravity (N-body) Simulation

It's gravy like the soup

As is, I just added a glTF model reader and gravity sim to [this](https://sotrh.github.io/learn-wgpu/) Rust [wgpu](https://wgpu.rs/) tutorial and got it to work in a browser.

<div>
<img align=top src="https://user-images.githubusercontent.com/26017543/236362359-534ef867-c157-4626-8112-5bd6304d5031.png"/>
</div>

## Usage

The project is compiled by wasm-pack from rust to an npm package.

```bash
$ pnpm i gravy-galaxy
```

Currently it builds for WebGL but that will be changed to webGPU when it's released.

## Resources

- [learn-wgpu - sotrh](https://sotrh.github.io/learn-wgpu/)
- [Diagram of Galaxy - Bartolomeu Velho](https://en.wikipedia.org/wiki/File:Bartolomeu_Velho_1568.jpg)
- [Solar System 3D Models - NASA](https://solarsystem.nasa.gov/resources)
- [Mutual Attraction - The Coding Train](https://www.youtube.com/watch?v=GjbKsOkN1Oc)
