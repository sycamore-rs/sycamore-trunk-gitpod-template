# Maple Trunk Gitpod Template

This is template for [Maple](https://github.com/lukechu10/maple) + [Trunk](https://trunkrs.dev/) on [gitpod.io](https://gitpod.io).

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/lukechu10/maple-trunk-gitpod-template)

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository](https://github.com/thedodd/trunk).

### Running

Just click on the button above to start a workspace. Trunk will automatically be installed and a dev server will automatically be started.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

This template is intended to be used as a quick way to test out something using Maple without having to setup a local environnement.
