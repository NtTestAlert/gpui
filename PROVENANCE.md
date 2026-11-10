# GPUI 0.3.12 published snapshot

This standalone workspace imports the exact registry source archives below.
The crate directories retain the published licenses and notices. The core
package adds the `element_lifecycle` integration test target to its published
manifests. The root workspace and local core override are integration
scaffolding added downstream. The source changes are described below.

Both published manifests identify upstream Zed revision
`d2a4fb6ded7c470be25d1541281da3882495b65b`.
The packaging script is Longbridge GPUI Kit's `script/bump-gpui.ts`; published
packages contain adaptations and must not be replaced by a raw Zed checkout.

| Package | Registry archive | SHA-256 |
| --- | --- | --- |
| gpui-pre 0.3.12 | https://static.crates.io/crates/gpui-pre/gpui-pre-0.3.12.crate | 29db89613527079737f53752eab27fb65c9a7dd94979256c65a0e590c9fd34d8 |
| gpui-pre-windows 0.3.12 | https://static.crates.io/crates/gpui-pre-windows/gpui-pre-windows-0.3.12.crate | 7201762420222d5b5ce5bdf0056996199212a3349bfb2457a3a0238c688681f3 |

Both archive checksums were verified against crates.io version metadata before
import. Both packages declare Apache-2.0. Preserve each package's license and
notices. All unmodified sibling packages remain registry dependencies with their
published exact `=0.3.12` requirements. The application workspace must override both
selected package names at its root to unify direct and transitive core dependencies.

For local work use `[patch.crates-io]` path overrides for `gpui-pre` and
`gpui-pre-windows`. For reproducible distribution commit reviewed changes to a
reachable snapshot fork and replace both overrides with the same immutable Git
revision. Keep package names, library names and versions unchanged.

## Downstream fixes

- Prune empty render-dependency entries while preserving observers and
  multi-window invalidation. Tests and builds enabling `test-support` can use
  `App::tracked_render_entity_count` to check the number of indexed entity keys.
- Own elements created outside a frame so their captured values are dropped;
  keep arena allocation for elements constructed during a frame.
- Retain the exact DirectWrite shaped face, including simulated styles and
  variable axes, and reuse stable rasterization IDs for equivalent faces.

Run the lifecycle fixtures and the Windows DirectWrite fixtures from the workspace root.
The Windows tests use installed Arial, Bahnschrift, Consolas and Segoe UI fonts.

```sh
cargo test -p gpui-pre --features test-support --test element_lifecycle
cargo test -p gpui-pre-windows --features test-support --lib direct_write:: -- --test-threads=1
```
