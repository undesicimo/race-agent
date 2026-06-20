# MCP

The MCP server is for analysis, not ingestion.

Initial tool set:

- `list_sessions`
- `get_session_summary`
- `list_laps`
- `get_lap_summary`
- `get_lap_telemetry`
- `compare_laps`
- `find_best_sector`
- `analyze_braking_zone`
- `analyze_throttle_application`
- `find_consistency_issues`

The server should reuse `packages/database` and `packages/analysis`.
