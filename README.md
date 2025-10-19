# cpuburn: a rust tool for maximizing your CPU usage

This tool will take all your cores and use all of them. Nice for testing stress
as well as CPU throttling (for instance in a VM or container environment). It's
also useful for validating metrics collectors such as the `prometheus` agent
`node-exporter`.

This tool is named after the old `cpuburn` Linux tool. I just wanted to whip up
something I could compile easy for myself. Hope you find it useful!

## Installation

```
cargo install cpuburn
```

## Usage

```
cpuburn
```

Combine with tools like `renice` and `tasksel` to really test how your system behaves when tapped out.

Press ^C to terminate it.

## Dockerfile

This repository also contains Dockerfiles which utilize staged builds to
give you a very small container to launch it into e.g. your Kubernetes cluster.

`Dockerfile` builds the source repository; `Dockerfile.release` builds from
[crates.io](https://crates.io/crate/cpuburn).

## License

MIT

## Author

Erik Hollensbe <git@hollensbe.org>
