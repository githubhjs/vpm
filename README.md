# Verilog Package Manager (VPM)
[![release](https://github.com/getinstachip/vpm/actions/workflows/release.yml/badge.svg)](https://github.com/getinstachip/vpm/actions/workflows/release.yml)
![downloads](https://img.shields.io/github/downloads/getinstachip/vpm/total?logo=github&logoColor=white&style=flat-square)

VPM is a package manager for Verilog projects, written in Rust. It's designed to simplify the management of IP cores and dependencies in hardware design workflows.

## Features

- Manage Verilog project dependencies
- Install packages from various sources (GitHub, local repositories, etc.)
- Auto-download specialized IP optimized for your use case (coming soon!)

## Installation

To install VPM, you don't need any dependencies! Just run the following command:

```bash
curl -sSfL https://raw.githubusercontent.com/getinstachip/vpm/main/install.sh | sh
```

After installation, you can use the `vpm` command in any terminal.

### Basic Commands

- Install a package: `vpm install <author/repo_name>`
- Include a module and its sub-modules: `vpm include <module_name.v> <author/repo_name>`

## Very useful stuff

### vpm include "top_module"
After running `vpm include "top_module.v"`, the Verilog Package Manager parses the file and downloads all the submodules too. It generates .vh files and handles synthesis collateral.

Example: running `vpm include pfcache.v` finds all dependences and includes/configures them for you.
```
your_project/
├─ vpm_modules/
│  ├─ pfcache/
│     ├─ pfcache.v
│     ├─ pfcache.vh
│     ├─ ffetch/
│     │  ├─ ffetch.v
│     │  └─ ffetch.vh
│     ├─ fwb_module/
│     │  ├─ fwb_module.v
│     │  └─ fwb_module.vh
│     └─ .v
└─ sim/
   └─pfcache_tb.v
```

## Configuration

### vpm.toml

Example `vpm.toml` file:

```yaml
[repositories]
https://github.com/ZipCPU/zipcpu = "ee644d4"
https://github.com/bensampson5/libsv = "c5aff5d"
https://github.com/alexforencich/verilog-pcie = "25156a9"

[modules]
pfcache.v
axilops.v
```

## Support

For issues and feature requests, please email sathvikr@getinstachip.com.
