<center>

![logo](assets/logo.svg)

# FabricUI (WIP)

A light-weight gui framework using layered window in Rust.

</center>

## Why this?

This library is designed to be light and fast, which is small application size,
low memory costing and nice render perfomance only using cpu rendering.

## What do we have now?

A lot of things are still work in progress.
Currently support Windows 7 and later, will try to support another system like Linux or MacOS (If I can have a Mac?).
Check the progress tab to view what is working on now.

## Want to help making this?

Any PR are welcomed! But be aware of the mess code made by myself :P

## Thanks these libraries :D

[`tiny-skia`](https://github.com/RazrFalcon/tiny-skia): Fast and small skia port to Rust, use it for main gui rendering.
[`font-kit`](https://github.com/servo/font-kit): A font finding and rasterizing library, use it for text rendering.
[`winapi`](https://github.com/retep998/winapi-rs): Gives us a lot of win32 api to make a layered window.
