import type { Fruit, FruitState } from "../../../../declarations/gaia/gaia.did";
import { Principal } from "@dfinity/principal";

export const fruits: Fruit[] = [
    {
        "id": BigInt(5),
        "title": "User Profile Management System",
        "description": `Implement enterprise-grade user profile system with:
- Multi-factor authentication integration
- GDPR-compliant data storage
- Role-based access control (RBAC)
- Audit logging for all profile changes
- Social media account linking
- Passwordless login options
- Data export in JSON/CSV formats
- Real-time activity monitoring
- Automated account cleanup (30d inactivity)
- SCIM 2.0 provisioning support`,

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(5000),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(6),
        "title": "Authentication Flow Enhancement",
        "description": "Improve OAuth2 integration with support for multi-factor authentication and session management",

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(5000),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(7),
        "title": "Performance Optimization Dashboard",
        "description": "Create monitoring dashboard with real-time metrics visualization and performance analytics",

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(3000),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(8),
        "title": "API Rate Limiting Implementation",
        "description": "Design and implement rate limiting system for public APIs using token bucket algorithm",

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(800),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(9),
        "title": "Data Model Schema Migration",
        "description": `Build advanced database migration framework with:
- Versioned migration scripts (SQL/NoSQL)
- Automatic dependency resolution
- Dry-run simulation mode
- Cross-database engine compatibility
- Atomic deployment rollbacks
- Migration history tracking
- CLI and GUI interfaces
- Pre-deployment validation checks
- Data transformation pipelines
- Multi-environment promotion workflows`,

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(4500),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(10),
        "title": "Notification Service Integration",
        "description": "Build real-time push notification system supporting email/SMS/webhook channels",

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(2500),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(11),
        "title": "User Data CRUD Operations",
        "description": "Develop comprehensive user data management system with:\n- Secure CRUD API endpoints\n- Role-based access control\n- Data validation rules\n- Audit logging\n- Bulk import/export functionality\n- Soft delete implementation\n- Database indexing optimization\nImplement using RESTful principles with OpenAPI documentation",

        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(1200),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    },
    {
        "id": BigInt(12),
        "title": "Distributed Tracing Implementation",
        "description": `Design and deploy enterprise-grade distributed tracing solution featuring:
- OpenTelemetry SDK integration for 20+ microservices
- Context propagation across HTTP/gRPC/Kafka
- Custom span attributes for business metrics
- Trace sampling configuration (head-based & tail-based)
- Integration with Jaeger and Prometheus
- Performance optimization for high-volume systems (10k+ spans/sec)
- Security controls for sensitive data redaction
- Grafana dashboard template for trace analysis
- Alerting rules for SLA violations (95th percentile latency >500ms)`,
        
        "creator": Principal.anonymous(),
        "created_at": BigInt(new Date().getTime()),

        "rewards": BigInt(6800),
        "deposit": BigInt(20),
        "state": { "open": null },
        "claimed_by": [],
    }
]