# Ground Trace Development Log

## 2024-10-28 - Docker & Database Foundation

### Session Summary
First real development session. Went from "What is Docker?" to having a fully functioning PostgreSQL database with migrations running from my Rust backend.

### What I Learned

#### Docker Fundamentals
- **Container vs Image**: Image is like a class, container is an instance
- **Docker Compose**: YAML file that defines multiple containers and how they connect
- **Port mapping**: `5432:5432` means "map container port 5432 to host port 5432"
- **Health checks**: Let Docker know when a service is actually ready (not just started)
- **Docker Desktop requirement**: The engine must be running for any docker commands to work

#### PostgreSQL in Docker
- Created using `image: postgres:16`
- Configured via environment variables (POSTGRES_USER, POSTGRES_PASSWORD, POSTGRES_DB)
- Data persists in Docker volumes even when container stops
- Can access with `docker compose exec database psql -U postgres -d groundtrace`

#### SQLx Migrations
- **When they run**: During Rust app startup via `db::run_migrations(&pool)`
- **NOT** when Docker starts - Docker just provides empty database
- **Migration files**: SQL in `backend/migrations/` folder
- **Tracking**: SQLx creates `_sqlx_migrations` table to track what's been run
- **Idempotent**: Safe to run multiple times, won't duplicate

#### Rust + Database Connection
- Use `dotenvy` crate to load `.env` file
- `DATABASE_URL` format: `postgres://user:password@host:port/database`
- `PgPool` is the connection pool SQLx uses
- Must add `.env` to `.gitignore` to avoid committing secrets

### What I Built
- [X] docker-compose.yml with PostgreSQL service
- [X] .env file with DATABASE_URL
- [X] backend/src/db.rs with connection and migration helpers
- [X] backend/src/main.rs that connects to DB and runs migrations
- [X] /healthz endpoint (basic test that server is running)

### Mental Models That Helped

**Multi-terminal workflow:**
- Terminal 1: `docker compose up` - keeps database running, shows logs
- Terminal 2: `cargo run` - runs my Rust app
- Terminal 3: For ad-hoc commands (testing, database queries, etc.)

**Separation of concerns:**
- Docker: "I provide infrastructure (database server)"
- Rust app: "I define the schema and logic (migrations, queries)"

### Problems I Solved

**Problem**: `docker compose exec db` said "service not running"
**Cause**: Docker Desktop wasn't started yet
**Solution**: Open Docker Desktop, wait for "Engine running"

**Problem**: Service name confusion (`db` vs `database`)
**Cause**: Changed service name in docker-compose.yml but forgot in commands
**Solution**: Keep names consistent - decided on `database` for now

**Problem**: Wasn't sure when migrations ran
**Experiment**: Deleted database, started Docker (no tables), then ran Rust app (tables appeared!)
**Learning**: App owns the schema, not the database

### Code Snippets to Remember

**Database connection pattern:**
```rust
let pool = db::connect_from_env().await?;
db::run_migrations(&pool).await?;
```

**Checking tables in PostgreSQL:**
```sql
\dt  -- list tables
\q   -- quit
```

**Docker commands I'll use often:**
```bash
docker compose up           # Start services (attached)
docker compose up -d        # Start services (detached/background)
docker compose down         # Stop and remove containers
docker compose ps           # List running containers
docker compose logs         # View logs
docker compose exec database psql -U postgres -d groundtrace  # Connect to DB
```

### Questions Still Have
- How do I handle database migrations in production? Can I roll back?
- What happens to the data if I delete the container?
- How do I back up the database?
- What's a connection pool and why do I need one?

### Next Session Goals
- [ ] Build `/api/sats` endpoint that queries the database
- [ ] Learn how to share database pool across Axum routes
- [ ] Understand Axum handler patterns (async functions that return responses)
- [ ] Return JSON from database query

### Resources That Helped
- Claude's Docker explanation (containers vs images analogy)
- SQLx documentation on migrations
- Realizing I could test with `\dt` in psql to see if tables exist

### Time Spent
~3 hours (including Docker installation and troubleshooting)

### Feeling
Invested, I make small progress that feels good, and actually learn well when I am struggling with writing code and making it work. Using tools to learn and accelerate learning has been a breath of fresh air. I dont want to have AI write my code, this will help make me better overall.