# GPUI 0.3.6 published snapshot

This standalone workspace imports the exact registry source archives below.
The crate directories retain their normalized published `Cargo.toml`, original
`Cargo.toml.orig`, licensing, notices and source without modification. The root
workspace and local core override are integration scaffolding added downstream.

Upstream Zed revision: `bcf6582ce3500df93a8a39366640173e6786cea6`.
The packaging script is Longbridge GPUI Kit's `script/bump-gpui.ts`; published
packages contain adaptations and must not be replaced by a raw Zed checkout.

| Package | Registry archive | SHA-256 |
| --- | --- | --- |
| gpui-pre 0.3.6 | https://static.crates.io/crates/gpui-pre/gpui-pre-0.3.6.crate | a0437c0b83e636a92bd1a39fa1d05fb632ae671289537497b35871ffbe231b84 |
| gpui-pre-windows 0.3.6 | https://static.crates.io/crates/gpui-pre-windows/gpui-pre-windows-0.3.6.crate | 27662e65bfcf4445d2cccfad91f1570ec07f4b1b5de98400a9f207fd19c464cf |

Both packages declare Apache-2.0. Preserve each package's license and notices.
All unmodified sibling packages remain registry dependencies with their published
exact `=0.3.6` requirements. The application workspace must override both selected
package names at its root to unify direct and transitive core dependencies.

For local work use `[patch.crates-io]` path overrides for `gpui-pre` and
`gpui-pre-windows`. For reproducible distribution commit reviewed changes to a
reachable snapshot fork and replace both overrides with the same immutable Git
revision. Keep package names, library names and versions unchanged.

## Element lifetime

During a window draw, GPUI allocates erased elements (`AnyElement`) in the
active frame arena. Clearing that arena runs their destructors together.
Outside a draw, this fork stores each erased element in an owning box. Dropping
a prebuilt tree releases its children and captured values, including when the
tree becomes a child of a frame-allocated element. The published snapshot put
off-frame elements in a thread-local arena that retained them until thread exit.
