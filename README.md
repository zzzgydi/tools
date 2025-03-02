<p align="center">
	<img alt="Rome's logo depicting an ancient Roman arch with the word Rome to its side" src="assets/PNG/logo_transparent_outlined.png" width="700">
</p>

# Rome is currently being rewritten in Rust. Read more about it in our [latest blog post](https://rome.tools/blog/2021/09/21/rome-will-be-rewritten-in-rust.html).
# The documentation below is out of date and available for posterity.

<!-- This intro is synced with the website via the `./rome run scripts/generate-files/website-intro` script. Make sure you run it after modifying anything between these comments. -->
<!-- INTRO START -->
**Rome** is a linter, compiler, bundler, and [more](https://rome.tools/#development-status) for JavaScript, TypeScript, JSON, HTML, Markdown, and CSS.

**Rome** is designed to replace [Babel](https://babeljs.io/), [ESLint](https://eslint.org/), [webpack](https://webpack.js.org/), [Prettier](https://prettier.io/), [Jest](https://jestjs.io/), and others.

**Rome** unifies functionality that has previously been separate tools. Building upon a shared base allows us to provide a cohesive experience for processing code, displaying errors, parallelizing work, caching, and configuration.

**Rome** has strong conventions and aims to have minimal configuration. Read more about our [project philosophy](https://rome.tools/#philosophy).

**Rome** is written in [TypeScript](https://www.typescriptlang.org/) and runs on [Node.js](https://nodejs.org/en/). **Rome** has zero dependencies, and has largely been written from scratch. See [credits](https://rome.tools/credits) for more information.

**Rome** is [MIT licensed](https://github.com/rome/tools/tree/main/LICENSE) and moderated under the [Contributor Covenant Code of Conduct](https://github.com/rome/tools/tree/main/CODE_OF_CONDUCT.md).
<!-- INTRO END -->

## Status

The current area of focus is **linting**. See the umbrella task [#20](https://github.com/rome/tools/issues/20) for tracking.

## Getting Started

To setup Rome in a project, all you need is a `rome.json` file.

```bash
$ mkdir hello-world
$ cd hello-world
$ rome init
```

This file is used to configure Rome and indicates the boundaries of your project.

See [Getting Started](https://rome.tools/#getting-started) guide for more usage instructions.

## Philosophy

The project philosophy can be found on our [website](https://rome.tools/#philosophy).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](./CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://discord.gg/rome). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
