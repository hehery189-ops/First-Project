# Full-Stack SvelteKit + Rust Dashboard

Monorepo starter untuk dashboard modern dengan **SvelteKit (frontend)** dan **Rust Axum (backend)**.

## Struktur Proyek

```
.
├── frontend/        # SvelteKit app
├── backend/         # Rust API server
├── docker-compose.yml
└── README.md
```

## Fitur Utama

- JWT authentication (register/login)
- Protected routes di backend
- CRUD items
- Role-based access (role disimpan di DB + JWT)
- Dockerized setup

## Menjalankan Lokal (Tanpa Docker)

### Backend

```bash
cd backend
cp .env.example .env
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

## Menjalankan Dengan Docker

```bash
docker compose up --build
```

Frontend akan tersedia di `http://localhost:5173` dan backend di `http://localhost:8080`.

## Skema Database (PostgreSQL)

Jalankan SQL ini di database PostgreSQL Anda:

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid PRIMARY KEY,
  email text UNIQUE NOT NULL,
  password_hash text NOT NULL,
  role text NOT NULL DEFAULT 'User',
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE items (
  id uuid PRIMARY KEY,
  owner_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title text NOT NULL,
  description text,
  created_at timestamptz NOT NULL DEFAULT now()
);
```

## Endpoint API

- `POST /auth/register`
- `POST /auth/login`
- `GET /users/me` (protected)
- `GET /items` (protected)
- `POST /items` (protected)
- `PUT /items/:id` (protected)
- `DELETE /items/:id` (protected)

## Catatan

- JWT disimpan di frontend (localStorage atau cookie sesuai kebutuhan proyek).
- Ubah `JWT_SECRET` untuk environment production.
