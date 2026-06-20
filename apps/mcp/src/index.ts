import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";

const server = new McpServer({
  name: "sim-telemetry",
  version: "0.1.0"
});

server.tool(
  "list_sessions",
  "List telemetry sessions available for analysis.",
  {
    limit: z.number().int().min(1).max(100).default(20)
  },
  ({ limit }) => ({
    content: [
      {
        type: "text",
        text: `Database-backed session listing is not implemented yet. Requested limit: ${limit}.`
      }
    ]
  })
);

const transport = new StdioServerTransport();
await server.connect(transport);
