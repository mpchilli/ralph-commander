# Shell Style Guide
- **Safety:** Always use set -e and set -u in scripts to prevent silent failures.
- **Portability:** Use POSIX-compliant syntax where possible, or explicitly target ash.
- **Variables:** Use quotes around variables to prevent word splitting and globbing.
- **Documentation:** Comment the purpose of non-trivial logic and external dependencies.
