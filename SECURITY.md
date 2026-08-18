# Security

We strive to make our software as secure as possible. However, security vulnerabilities are inevitable over time. This document describes our process for handling security issues according to a responsible disclosure policy.

## Supported versions

We only support the latest version of this project. The latest version is defined by our main branch.

## Finding security issues

You are welcome to search for security issues on our systems. However, please adhere to the following rules:

- Please do not degrade services for other users
- Please do not incur costs (e.g., for API requests)
- Try to self-host systems you are investigating (after all, it's free software)
- Do not use your findings on systems you do not own

## Reporting security issues

### Contact points

Please report security vulnerabilities using one of these methods:

- By email, preferably encrypted via GPG ([GPG Public Key](https://mixxplorer.de/keys/gpg/security@mixxplorer.de.asc)) to `security@mixxplorer.de`
- By Signal via `@lmm.99`
- Also check the [security.txt of mixxplorer.de](https://mixxplorer.de/.well-known/security.txt)

We prefer contact via email. Please use English or German.

### Response policies

We typically acknowledge your message within 24 hours and guarantee acknowledgment within 72 hours.

Typically, we provide a fix within 30 days. In rare cases, this timeframe may extend to 90 days. We will keep you informed about the progress throughout the process.

### Requested information

In your report, please include the following:

- Details about the tested system/environment
- Vulnerability outcome (e.g., gained access, sensitive information, etc.)
- Steps to reproduce the security vulnerability (if available, e.g., exploit scripts)
- (Optional) category and severity of the vulnerability

## Acknowledgement

We are open to acknowledging reporters in our public communications. Please indicate your preference when submitting your report.

### Bounty

Unfortunately, we cannot offer bounties at this time. However, we sincerely thank you for every notification and recognize that compiling a security report may take considerable time and effort.

## Disclosure

We disclose all security-related issues after providing a fix and ensuring users have had the opportunity to upgrade. Please coordinate disclosure with us and do not share details about a security vulnerability before we announce it publicly.
