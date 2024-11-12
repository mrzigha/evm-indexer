# Security Policy

## Supported Versions

Currently supported versions of EVM-Indexer:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Updates

Security updates will be released as patch versions. We strongly recommend keeping your installation up to date with the latest patch version.

## Reporting a Vulnerability

If you discover a security vulnerability in EVM-Indexer, please follow these steps:

1. **DO NOT** disclose the vulnerability publicly.
2. Open a private security advisory.
   - Describe the vulnerability
   - Include steps to reproduce
   - If possible, include a fix or suggestions for fixing
3. You will receive a response within 48 hours.
4. Once the vulnerability is confirmed:
   - A fix will be developed
   - A new version will be released
   - The vulnerability will be publicly disclosed after users have had time to update

## Security Best Practices

When deploying EVM-Indexer:

1. **Environment Variables**
   - Never commit sensitive environment variables
   - Use secure secrets management
   - Rotate credentials regularly

2. **Network Security**
   - Run behind a reverse proxy
   - Use TLS/SSL for all connections
   - Limit access to metrics endpoints

3. **MongoDB Security**
   - Use authentication
   - Enable access control
   - Regular security updates
   - Proper network isolation

4. **RPC Endpoints**
   - Use secure WebSocket connections (WSS)
   - Use HTTPS for HTTP endpoints
   - Implement rate limiting
   - Monitor for abuse

5. **Docker Security**
   - Use official images only
   - Keep images updated
   - Implement resource limits
   - Use non-root users

## Secure Configuration

Example of secure configuration:
```toml
[general]
metrics_laddr = "127.0.0.1"  # Local access only
metrics_port = 9090

[database]
db_host = "mongodb"
db_port = 27017
db_name = "indexer"
```
```bash
# Use environment variables for credentials
EVM_INDEXER_DATABASE_USERNAME=...
EVM_INDEXER_DATABASE_PASSWORD=...
```

## Security Checklist

Before deploying to production:

- [ ] All environment variables are properly set
- [ ] MongoDB authentication is enabled
- [ ] Secure RPC endpoints are configured
- [ ] Metrics endpoint is properly secured
- [ ] Latest version is installed
- [ ] Logging is properly configured
- [ ] Monitoring is set up
- [ ] Backup strategy is in place
- [ ] Recovery procedures are documented

## Dependencies

We regularly update dependencies to patch security vulnerabilities. Users should:

1. Monitor security advisories
2. Update promptly when security patches are released
3. Regularly check for outdated dependencies

## Contact

For security concerns, contact:
- Discord: `@mrzigha`
- GitHub Security Advisory [Repository Security Tab]("https://github.com/mrzigha/evm-indexer/security/advisories")
