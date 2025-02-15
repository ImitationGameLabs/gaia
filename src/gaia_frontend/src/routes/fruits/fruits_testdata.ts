import type { Fruit, FruitState } from "../../../../declarations/gaia/gaia.did";
import { Principal } from "@dfinity/principal";

export const fruits: Fruit[] = [
    {
        "id": BigInt(5),
        "title": "User Profile Management System",
        "description": "Develop a comprehensive user profile system with editable fields, avatar upload, and privacy settings",

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
    }
]