# Security

Nexus Cerebri is pre-release research/development software.

Never commit secrets, credentials, production calendar data or personal training data. Treat planning/execution separation, permissions, redaction and idempotency as security boundaries.

Security-sensitive architecture changes require an ADR. A future implementation repository should define a private vulnerability-reporting channel before public production use.

## Foundation implementation boundary

The REST app binds to 127.0.0.1:3000 and exposes no execution route.
It has no production authentication. Client CPIR policy/capability fields are planning inputs,
not trusted execution credentials. Trusted application code must construct live authorization
from authenticated identity and current provider state before calling the executor.

The provided ledger and adapter are in-memory test tools. They do not survive process failure.
Production adapters must implement durable atomic claims, optimistic concurrency, reconciliation
and explicit retry policy. Never retry uncertain writes by clearing an idempotency key.
Partial execution is represented explicitly and does not imply rollback.

Tracing records a validated TraceId, not request bodies, event titles or descriptions.
HTTP/Node inputs and core search work have explicit bounds. The development API is not hardened
for hostile multi-user traffic. Repository checks detect selected credential patterns; they are
not a comprehensive secret audit. The project uses the MIT license; a private production disclosure channel still needs to be configured before production use.
