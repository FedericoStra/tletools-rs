# TLE-tools (Rust crate)

[![GitHub](https://img.shields.io/static/v1?label=github&message=FedericoStra/tletools-rs&color=brightgreen&logo=github)](https://github.com/FedericoStra/tletools-rs)
[![Crates.io](https://img.shields.io/crates/v/tletools?logo=rust)](https://crates.io/crates/tletools)
[![docs.rs](https://img.shields.io/docsrs/tletools?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBkPSJNNDg4LjYgMjUwLjJMMzkyIDIxNFYxMDUuNWMwLTE1LTkuMy0yOC40LTIzLjQtMzMuN2wtMTAwLTM3LjVjLTguMS0zLjEtMTcuMS0zLjEtMjUuMyAwbC0xMDAgMzcuNWMtMTQuMSA1LjMtMjMuNCAxOC43LTIzLjQgMzMuN1YyMTRsLTk2LjYgMzYuMkM5LjMgMjU1LjUgMCAyNjguOSAwIDI4My45VjM5NGMwIDEzLjYgNy43IDI2LjEgMTkuOSAzMi4ybDEwMCA1MGMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAzLjktNTIgMTAzLjkgNTJjMTAuMSA1LjEgMjIuMSA1LjEgMzIuMiAwbDEwMC01MGMxMi4yLTYuMSAxOS45LTE4LjYgMTkuOS0zMi4yVjI4My45YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43ek0zNTggMjE0LjhsLTg1IDMxLjl2LTY4LjJsODUtMzd2NzMuM3pNMTU0IDEwNC4xbDEwMi0zOC4yIDEwMiAzOC4ydi42bC0xMDIgNDEuNC0xMDItNDEuNHYtLjZ6bTg0IDI5MS4xbC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnptMjQwIDExMmwtODUgNDIuNXYtNzkuMWw4NS0zOC44djc1LjR6bTAtMTEybC0xMDIgNDEuNC0xMDItNDEuNHYtLjZsMTAyLTM4LjIgMTAyIDM4LjJ2LjZ6IiBzdHlsZT0iZmlsbDojZmZmZmZmIj48L3BhdGg+PC9zdmc+Cg==)](https://docs.rs/tletools)
[![MIT license](https://img.shields.io/crates/l/tletools)](https://github.com/FedericoStra/tletools-rs/blob/master/LICENSE)
<!-- [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/FedericoStra/tletools-rs/Rust)](https://github.com/FedericoStra/tletools-rs/actions/workflows/rust.yml) -->
![Lines of code](https://tokei.rs/b1/github/FedericoStra/tletools-rs?category=code)

`TLE-tools` is a small library to work with [two-line element
set](https://en.wikipedia.org/wiki/Two-line_element_set) files.

## Features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```toml
[dependencies]
tletools = {version = "0.x.y", default-features = false} 
```

## Purpose

The purpose of the library is to parse TLE sets into convenient `TLE` structures.

From [Wikipedia](https://en.wikipedia.org/wiki/Two-line_element_set):

> A two-line element set (TLE) is a data format encoding a list of orbital
elements of an Earth-orbiting object for a given point in time, the epoch.
The TLE data representation is specific to the [simplified perturbations
models](https://en.wikipedia.org/wiki/Simplified_perturbations_models) (SGP,
SGP4, SDP4, SGP8 and SDP8), so any algorithm using a TLE as a data source must
implement one of the SGP models to correctly compute the state at a time of
interest. TLEs can describe the trajectories only of Earth-orbiting objects.

Example:

```
ISS (ZARYA)
1 25544U 98067A   19249.04864348  .00001909  00000-0  40858-4 0  9990
2 25544  51.6464 320.1755 0007999  10.9066  53.2893 15.50437522187805
```

### TLE format specification

Some more or less complete TLE format specifications can be found on the following websites:

- [Wikipedia](https://en.wikipedia.org/wiki/Two-line_element_set#Format)
- [NASA](https://spaceflight.nasa.gov/realdata/sightings/SSapplications/Post/JavaSSOP/SSOP_Help/tle_def.html)
- [CelesTrak](https://celestrak.com/columns/v04n03/)
- [Space-Track](https://www.space-track.org/documentation#tle)

## Links

- Repository: https://github.com/FedericoStra/tletools-rs
- Documentation: https://docs.rs/tletools
- Releases: https://crates.io/crates/tletools
- Issue tracker: https://github.com/FedericoStra/tletools-rs/issues

