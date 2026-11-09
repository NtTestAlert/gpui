# GPUI 0.3.9 published snapshot

This standalone workspace imports the exact registry source archives below.
The crate directories retain their normalized published `Cargo.toml`, original
`Cargo.toml.orig`, licensing, notices and source without modification. The root
workspace and local core override are integration scaffolding added downstream.

Upstream Zed revision: `93d745668805029476e6d7fbb19652116fed51de`.
The packaging script is Longbridge GPUI Kit's `script/bump-gpui.ts`; published
packages contain adaptations and must not be replaced by a raw Zed checkout.

| Package | Registry archive | SHA-256 |
| --- | --- | --- |
| gpui-pre 0.3.9 | https://static.crates.io/crates/gpui-pre/gpui-pre-0.3.9.crate | 34f61c1cfd36fc9c436af09308c963822ae27a45a84feb21a77ba5c9e051f1c0 |
| gpui-pre-windows 0.3.9 | https://static.crates.io/crates/gpui-pre-windows/gpui-pre-windows-0.3.9.crate | 513d4f0fc7d596d3bdef2a73a524ffb83dabc95bf484f2855a698e7f431290a9 |

Both archive checksums were verified against crates.io version metadata before
import. Both packages declare Apache-2.0. Preserve each package's license and
notices. All unmodified sibling packages remain registry dependencies with their
published exact `=0.3.9` requirements. The application workspace must override both
selected package names at its root to unify direct and transitive core dependencies.

For local work use `[patch.crates-io]` path overrides for `gpui-pre` and
`gpui-pre-windows`. For reproducible distribution commit reviewed changes to a
reachable snapshot fork and replace both overrides with the same immutable Git
revision. Keep package names, library names and versions unchanged.
