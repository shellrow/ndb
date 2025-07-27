# ndb
Modular, embeddable network-related database library, 
designed for high-performance tools like scanners, monitors, and analyzers.

## Crates
### Protocols and Services
- [`ndb-oui`](./ndb-oui): OUI database with lookup interface.
- [`ndb-tcp-service`](./ndb-tcp-service): TCP service database with lookup interface.
- [`ndb-udp-service`](./ndb-udp-service): UDP service database with lookup interface.

### IP to ASN / Country
- [`ndb-ipv4-asn`](./ndb-ipv4-asn): IPv4 ASN database with lookup interface.
- [`ndb-ipv6-asn`](./ndb-ipv6-asn): IPv6 ASN database with lookup interface.
- [`ndb-ipv4-country`](./ndb-ipv4-country): IPv4 country database with lookup interface.
- [`ndb-ipv6-country`](./ndb-ipv6-country): IPv6 country database with lookup interface.

### Autonomous System and Country
- [`ndb-as`](./ndb-as): AS database with lookup interface.
- [`ndb-country`](./ndb-country): Country database with lookup interface.

## Features
- High-speed lookups
- Optional bundled datasets via cargo (default)features
- Support for custom datasets via runtime loading
- Structured interfaces
