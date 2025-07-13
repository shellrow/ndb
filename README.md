# ndb
Modular, embeddable network-related database library, 
designed for high-performance tools like scanners, monitors, and analyzers.

## Crates
- [`ndb-oui`](./ndb-oui): OUI database with lookup interface.
- [`ndb-tcp-service`](./ndb-tcp-service): TCP service database with lookup interface.
- [`ndb-udp-service`](./ndb-udp-service): UDP service database with lookup interface.
- [`ndb-core`](./ndb-core): Internal utilities shared across ndb crates

## Features
- High-speed lookups
- Optional bundled datasets via cargo (default)features
- Support for custom datasets via runtime loading
- Structured interfaces
