# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | ✅ Yes             |
| < 1.0   | ❌ No              |

## Reporting a Vulnerability

The SuperAlign team takes security seriously. If you find a security vulnerability, please do NOT open a public issue.

Instead, please report it via one of these channels:
1. **Email**: `security@superalign.io`
2. **GitHub**: Use the "Report a vulnerability" button under the "Security" tab.

### Process
1. You will receive an acknowledgment within 48 hours.
2. We will provide a preliminary assessment and timeline.
3. We follow a **90-day coordinated disclosure** policy.

## Sandboxing Notice
SuperAlign's **Plugin Runtime** is designed to isolate external logic. However, users should only run plugins from trusted sources. Similarly, the **WASM preprocessing layer** is sandboxed by the browser but processes sensitive data locally; verify your browser's security integrity when handling private genomes.
