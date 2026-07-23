# Security policy

Please report vulnerabilities privately to `hiwepy@gmail.com`.

Supported pre-1.0 code is the latest release line only. Security fixes may
include API changes when preserving the old behavior would remain unsafe.

Hutool-Rust does not enable legacy cryptography by default. Security-sensitive APIs
must expose explicit algorithms, limits, and validation policy. Known vulnerable
dependencies are not accepted into default features.

Include affected crate/version, reproduction steps and impact. Do not open a
public issue until coordinated disclosure is complete. See
`docs/security.md` for the threat model and caller responsibilities.
