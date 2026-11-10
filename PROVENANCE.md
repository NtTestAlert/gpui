# GPUI 0.3.10 published snapshot

This standalone workspace imports the exact registry source archives below.
The crate directories retain the published licenses and notices. The core
package adds the `element_lifecycle` integration test target to its published
manifests. The root workspace and local core override are integration
scaffolding added downstream. The source changes are described below.

Upstream Zed revision: `08cbb88a152c3751a43e853425a70e9f631a5918`.
The packaging script is Longbridge GPUI Kit's `script/bump-gpui.ts`; published
packages contain adaptations and must not be replaced by a raw Zed checkout.

| Package | Registry archive | SHA-256 |
| --- | --- | --- |
| gpui-pre 0.3.10 | https://static.crates.io/crates/gpui-pre/gpui-pre-0.3.10.crate | df4b5f8d0c278308df2f11e0bfe0212dd77d7848ce1bdc4b9d513334414396f6 |
| gpui-pre-windows 0.3.10 | https://static.crates.io/crates/gpui-pre-windows/gpui-pre-windows-0.3.10.crate | 80670f987b920fba019917b2e222ecf2b377f2a6d77872010c7d284c235837b3 |

Both archive checksums were verified against crates.io version metadata before
import. Both packages declare Apache-2.0. Preserve each package's license and
notices. All unmodified sibling packages remain registry dependencies with their
published exact `=0.3.10` requirements. The application workspace must override both
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
