# Monitoring Guide

EVM-Indexer provides comprehensive metrics for monitoring its operation. This guide explains available metrics and their interpretation.

## Metrics Endpoint

Metrics are available at: `http://localhost:9090/metrics`

## Available Metrics

### Event Processing Metrics

#### `indexer_events_processed_total`
- **Type**: Counter
- **Labels**: `chain`, `event_type`
- **Description**: Total number of events processed
- **Usage**: Monitor event processing throughput

#### `indexer_events_by_type`
- **Type**: Counter
- **Labels**: `chain`, `event_type`
- **Description**: Number of events by type
- **Usage**: Track distribution of event types

#### `indexer_events_received`
- **Type**: Counter
- **Labels**: `chain`
- **Description**: Total events received before processing
- **Usage**: Monitor input event flow

### Connection Metrics

#### `indexer_active_connections`
- **Type**: Gauge
- **Labels**: `chain`, `endpoint`, `transport_type`
- **Description**: Number of active RPC connections
- **Usage**: Monitor connection health and transport type (ws/http)

#### `indexer_endpoint_failures`
- **Type**: Counter
- **Labels**: `chain`, `endpoint`, `transport_type`
- **Description**: Number of RPC endpoint failures
- **Usage**: Track endpoint reliability by transport type

#### `indexer_endpoint_latency`
- **Type**: Histogram
- **Labels**: `chain`, `endpoint`, `transport_type`
- **Description**: RPC endpoint latency in seconds
- **Usage**: Monitor endpoint performance and compare WebSocket vs HTTP

#### `indexer_polling_interval`
- **Type**: Gauge
- **Labels**: `chain`, `endpoint`
- **Description**: Current polling interval for HTTP endpoints in seconds
- **Usage**: Monitor and tune HTTP polling performance

### Processing Metrics

#### `indexer_event_processing_duration`
- **Type**: Histogram
- **Labels**: `chain`, `event_type`
- **Description**: Time taken to process events
- **Usage**: Monitor processing performance

#### `indexer_last_block_height`
- **Type**: Gauge
- **Labels**: `chain`
- **Description**: Last processed block height
- **Usage**: Track synchronization progress

### Circuit Breaker Metrics

#### `indexer_circuit_breaker_trips`
- **Type**: Counter
- **Labels**: `chain`, `endpoint`
- **Description**: Number of circuit breaker activations
- **Usage**: Monitor system stability

## Alerting

Recommended alert thresholds:

1. **Connection Loss**
   - Metric: `indexer_active_connections{transport_type="ws"} + indexer_active_connections{transport_type="http"}`
   - Threshold: `== 0`
   - Duration: `5m`

2. **High Latency**
   - Metric: `indexer_endpoint_latency`
   - Threshold: `> 2s`
   - Duration: `5m`
   - Note: Consider different thresholds for WebSocket vs HTTP

3. **Processing Delays**
   - Metric: `indexer_event_processing_duration`
   - Threshold: `> 5s`
   - Duration: `5m`

4. **Frequent Circuit Breaks**
   - Metric: `rate(indexer_circuit_breaker_trips[5m])`
   - Threshold: `> 0.1`
   - Duration: `5m`

5. **HTTP Polling Performance**
   - Metric: `rate(indexer_events_received{transport_type="http"}[5m])`
   - Threshold: Depends on expected event rate
   - Duration: `5m`