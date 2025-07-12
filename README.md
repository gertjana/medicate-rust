# Medicate Rust

A Rust implementation of the Medicate medicine management API, providing the same functionality as the Scala/ZIO version.

## Features

- Medicine management (CRUD operations)
- Medicine schedules
- Dosage history tracking
- Redis-based persistence
- RESTful API with CORS support

## API Endpoints

### Medicines
- `POST /medicines` - Create a new medicine
- `GET /medicines` - Get all medicines
- `GET /medicines/:id` - Get medicine by ID
- `PUT /medicines/:id` - Update medicine
- `DELETE /medicines/:id` - Delete medicine
- `POST /medicines/:id/addStock?amount=X` - Add stock to medicine

### Schedules
- `POST /schedules` - Create a new schedule
- `GET /schedules` - Get all schedules
- `GET /schedules/:id` - Get schedule by ID
- `PUT /schedules/:id` - Update schedule
- `DELETE /schedules/:id` - Delete schedule
- `GET /schedules/daily/:date` - Get daily schedule

### Dosage History
- `POST /dosage-history` - Create dosage history entry
- `GET /dosage-history` - Get all dosage history
- `DELETE /dosage-history/:id` - Delete dosage history entry

## Environment Variables

- `PORT` - Server port (default: 8080)
- `REDIS_HOST` - Redis host (default: localhost)
- `REDIS_PORT` - Redis port (default: 6379)
- `RUST_LOG` - Log level (default: info)

## Running

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/medicate-rust
```

## Docker

```bash
docker build -t medicate-rust .
docker run -p 8080:8080 medicate-rust
``` 

if on an ARM architecture (MacOS M?) then add `--platform linux/amd64` to both build and run