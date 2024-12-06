# Troubleshooting Guide

If you encounter issues with EVM-Indexer, follow these steps to diagnose and resolve the problem.

## First Steps

1. **Check Logs**
   - Check the logs in your configured log directory
   - Look for ERROR or WARN level messages
   - Note any relevant error messages or stack traces

2. **Check Metrics**
   - Access the metrics endpoint (default: http://localhost:9090/metrics)
   - Check for:
     - Connection status
     - Event processing rates
     - Error counts
     - Circuit breaker status

3. **Check Health Endpoint**
   - Access the health endpoint (default: http://localhost:9090/health)
   - Verify the status of all components

## Common Issues

### Connection Issues
- Verify RPC endpoint availability
- Check network connectivity
- Verify WebSocket/HTTP endpoints are correct
- Check RPC rate limits
- For HTTP endpoints, verify if polling interval is appropriate
- For WebSocket endpoints, check for connection stability
- Monitor event latency differences between HTTP and WebSocket

### Transport-Specific Issues

#### WebSocket Issues
- Check if WebSocket port is accessible
- Verify WebSocket protocol (wss:// vs ws://)
- Check for connection timeouts
- Monitor reconnection attempts

#### HTTP Issues
- Verify endpoint supports necessary eth_ methods
- Check rate limiting on HTTP endpoints
- Monitor polling interval effectiveness
- Verify response times are within acceptable range

### MongoDB Issues
- Verify MongoDB connection string
- Check MongoDB credentials
- Ensure database permissions are correct
- Verify MongoDB is running

### Event Processing Issues
- Verify ABI file is correct
- Check contract address
- Verify starting block number
- Check for event signature matches

## Opening an Issue

If you cannot resolve the issue, please open a GitHub issue with:

1. **Environment Information**
   - OS and version
   - Rust version
   - Docker version (if applicable)
   - MongoDB version

2. **Configuration**
   - Sanitized configuration file
   - Environment variables (without sensitive data)

3. **Logs**
   - Relevant log excerpts
   - Error messages
   - Stack traces

4. **Metrics**
   - Current metric values
   - Any abnormal patterns observed

5. **Steps to Reproduce**
   - Detailed steps to reproduce the issue
   - Expected vs actual behavior

## Additional Support

For urgent issues or if you need direct assistance:
- Open a GitHub issue
- Contact maintainers on Telegram