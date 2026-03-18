# mvr-gdtf

A Rust library for parsing and reading [MVR](https://www.gdtf.eu/mvr/prologue/introduction/) and [GDTF](https://www.gdtf.eu/gdtf/prologue/introduction/) files.

> ⚠️ **Warning** > This library is in early development. APIs, features, and behavior may change frequently and without notice.

## Overview

*MVR* (My Virtual Rig) and *GDTF* (General Device Type Format) are open standards used to describe lighting rigs and fixtures in entertainment production. While MVR files contain scene and rig data, GDTF files define the specific characteristics and geometry of individual fixtures.

Because these formats support thousands of devices across multiple manufacturers, their data structures are large and rely heavily on optional fields. This means it's non-trivial to read commonly used data like the channel count of a fixture's DMX mode. This often makes directly reading the structures verbose and difficult to manage.

`mvr-gdtf` abstracts this complexity by providing lookup tables and high-level helper functions. The goal is to let you extract the data you actually need without navigating the deep, nested specifications of the underlying XML. Though, if you want to manually find anything defined in the description files, you can.

<details>
    <summary><strong>Note:</strong> This library is currently <strong>read-only</strong>. Modifying or re-serializing MVR/GDTF data is not supported. <em>Why?</em></summary>

    This library is designed for parsing and extracting data from MVR and GDTF files, not for editing or generating them. Adding support for modification and re-serialization would mean managing the lookups would become a lot more complicated (and in some cases slower). In the future, I might reconsider adding serialization support if I find the time and a nice way to handle this.
</details>

## Cargo Features

By default, no features are selected.

- `gdtf`: Enable parsing/reading GDTF files.
- `mvr`: Enable parsing/reading MVR files (uses `gdtf`).

## Beta Release Roadmap
**MVR-xchange (TCP Mode of protocol)**
- [x] Automatically join stations in mDNS service.
- [x] Purge stations that have timed out.
- [ ] Follow API guidelines
- [ ] Sync API wrappers (maybe with `flume`?)
- [ ] Documentation
- [x] Handle `MVR_JOIN`
- [x] Handle `MVR_LEAVE`
- [ ] Handle `MVR_COMMIT`
- [ ] Handle `MVR_REQUEST`
- [ ] Handle `MVR_NEW_SESSION_HOST`

**MVR and GDTF**
- [x] Completely parse shared files into Rust data types.
- [ ] Completely parse GDTF files into Rust data types.
- [ ] Completely parse MVR files into Rust data types.
- [ ] Resource files management.
- [ ] Add methods to get values directly from the parsed data (Maybe mirror [libMVRgdtf](https://github.com/mvrdevelopment/libMVRgdtf)).
- [ ] Add lookups to get computed values (like channel counts or absolute DMX offsets) quickly.
- [ ] Unit tests (Maybe mirror [libMVRgdtf](https://github.com/mvrdevelopment/libMVRgdtf)'s testsuite).
- [ ] Add usage examples.
- [ ] Documentation.

## Contributing
Contributions are welcome. If you find a file that this library fails to parse correctly or want to request a feature or suggest a change, feel free to open an issue!

## License

This project is dual-licensed under:

- MIT License
- Apache License, Version 2.0

You may choose either license to govern your use of this project.
See the LICENSE-MIT and LICENSE-APACHE files for details.
