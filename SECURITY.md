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

## ffmpeg Subprocess Isolation (v0.1.2+)

Since v0.1.2, the CLI may invoke `ffmpeg` as a subprocess to
extract audio from video containers and to fall back when native
OGG/Opus decoding fails. The subprocess is invoked with the
following hardening guarantees:

- env_clear: the child process inherits no environment variables
  from the parent. Only an explicit allowlist of `PATH`, `HOME`,
  `TMPDIR`, `LANG`, `LC_ALL` is added back. This prevents
  accidental leakage of secrets via ffmpeg error logs.
- setsid (Unix) / CREATE_NEW_PROCESS_GROUP (Windows): the child
  runs in its own process group. SIGINT delivered to the parent
  CLI does not silently propagate to ffmpeg, allowing the parent
  to perform graceful shutdown while leaving the child to its
  own lifecycle.
- Kill-on-drop: the child handle is wrapped in a SafeChild
  guard with a Drop implementation that sends SIGKILL (Unix) or
  TerminateProcess (Windows) on parent panic. This prevents
  zombie ffmpeg processes.
- Bounded timeout: a default 180s timeout per invocation. On
  timeout, the child is killed and `Error::VideoExtractionFailed`
  is returned.
- WAV output validation: the extracted WAV is validated
  post-process. The header must be `RIFF...WAVE` and the chunk
  size must match the file size minus 8. This catches the class
  of bugs where ffmpeg exits 0 but produces an empty or truncated
  file.
- Temp cleanup: the temp WAV file is removed via a Drop guard
  even if decode panics or the process is interrupted.
- Magic bytes validation BEFORE ffmpeg invocation: the input
  file is sniffed for video container magic bytes before ffmpeg
  is invoked. This refuses to invoke ffmpeg on renamed
  non-video files.

The subprocess is invoked via `std::process::Command` with
`env_clear()` and `pre_exec` (Unix) or `creation_flags`
(Windows). The child is NOT linked into the binary. ffmpeg must
be installed separately; if it is not, the CLI returns exit
code 69 with a clear install hint.

## Known Limitations

- The 3GB Whisper model is loaded entirely into unified memory
- VAD (Voice Activity Detection) may miss quiet speech
- Hash-based deduplication is not performed across invocations
- Audio is held in memory during transcription; large files increase
  peak memory consumption
- ffmpeg is an external binary, not bundled. Behavior depends on
  the user-installed version of ffmpeg
- Temp WAV files are written to the system temp directory; users
  with restricted temp directories may need to override via
  `TMPDIR` environment variable

## Cryptography

This project uses standard cryptographic primitives from the Rust
ecosystem:

- `sha2` for model integrity verification
- `rustls` for TLS connections
- `uuid` v7 for correlation identifiers

No custom cryptographic code is included in the project.

## Past Security Advisories

None published to date.
