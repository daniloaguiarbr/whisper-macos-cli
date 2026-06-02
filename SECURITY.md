# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 0.1.x   | Yes                |
| < 0.1   | No                 |

## Reporting a Vulnerability

Please report security vulnerabilities through GitHub Security Advisories
at https://github.com/daniloaguiarbr/whisper-macos-cli/security/advisories/new

DO NOT open a public issue for security vulnerabilities.

### What to Include

- Description of the vulnerability and its impact
- Steps to reproduce or a proof of concept
- Affected versions
- Your assessment of severity (low, medium, high, critical)
- Any known mitigations

### Response Time

- Triagem inicial: within 72 hours of submission
- Atualização de status: within 7 days
- Patch release: within 30 days for high or critical vulnerabilities

### Coordinated Disclosure

We follow coordinated disclosure. We request that you do not disclose
the vulnerability publicly until we have released a fix or 90 days
have elapsed, whichever is sooner.

## Security Guarantees

- All model downloads are verified via SHA256 hash
- HTTPS-only communication with Hugging Face
- TLS certificate validation is enabled by default
- No telemetry or phone-home behavior
- All audio data is processed locally on the user's machine
- Transcription output is not transmitted to any external service

## Threat Model

The CLI is designed to run as a local user process on macOS Apple
Silicon. It is NOT designed to:

- Be exposed as a network service
- Process untrusted model files (verify hashes before loading)
- Run as root or with elevated privileges
- Operate in a multi-tenant environment

## Known Limitations

- The 3GB Whisper model is loaded entirely into unified memory
- VAD (Voice Activity Detection) may miss quiet speech
- Hash-based deduplication is not performed across invocations
- Audio is held in memory during transcription; large files increase
  peak memory consumption

## Cryptography

This project uses standard cryptographic primitives from the Rust
ecosystem:

- `sha2` for model integrity verification
- `rustls` for TLS connections
- `uuid` v7 for correlation identifiers

No custom cryptographic code is included in the project.

## Past Security Advisories

None published to date.
