# GPUI 0.2.2

This branch is based on Zed revision `69e2130295c2649963eb639fc70b4f2ee8ea1624` and the published `gpui` 0.2.2 crate, licensed under Apache-2.0. The package metadata marks its source tree dirty. The crates.io archive has SHA-256 `979b45cfa6ec723b6f42330915a1b3769b930d02b2d505f9697f8ca602bee707`. The GPUI manifest uses the published package's normalized registry dependencies; upstream examples remain in the repository but their Cargo targets are omitted.

The Windows DirectWrite renderer stores the exact fallback face returned by shaping in its font cache. Resolving a fallback face by family could repeatedly allocate font IDs and rasterize glyphs using a different face.

The app now removes render dependencies for unmounted entities and closed windows. Notifying an entity outside a window continues to notify its observers without allocating window metadata.

When updating this dependency, run `cargo test -p atrophy-ui --test windows_font_fallback` on Windows and `cargo test -p atrophy-ui --features interaction-tests --test interactions render_dependencies` on any platform.

GPUI compiles its Windows shaders with the Windows SDK's `fxc.exe`. If the build script cannot find the compiler, set `GPUI_FXC_PATH` to the executable's full path.
