# Finance Tracker API

A small backend API for tracking user transactions. It lets a user create an account, log in, add transactions, list their transactions, and check their current balance.

Live service:

```text
https://finance-tracker-vbn0.onrender.com
```

Health check:

```text
https://finance-tracker-vbn0.onrender.com/health
```

## Tech Stack

- Rust
- Actix Web
- PostgreSQL
- SQLx
- JWT authentication
- Argon2 password hashing
- Docker
- Render
- Neon Postgres

## Routes

### Health Check

```http
GET /health
```

Returns a simple response showing that the server is running.

### Create User

```http
POST /users
```

Request body:

```json
{
  "username": "optional_username",
  "email": "finance_email@example.com",
  "password": "finance_password_123"
}
```

`username` is optional.

Response:

```json
{
  "name": "optional_username",
  "email": "finance_email@example.com",
  "timestamp": "2026-06-07T12:00:00"
}
```

### Log In

```http
POST /login
```

Request body:

```json
{
  "email": "finance_email@example.com",
  "password": "finance_password_123"
}
```

Response:

```json
{
  "access_token": "jwt-token-here",
  "token_type": "Bearer"
}
```

Use the token in protected routes:

```http
Authorization: Bearer jwt-token-here
```

### Add Transaction

```http
POST /transactions
```

Requires a bearer token.

Request body:

```json
{
  "amount": 25.50,
  "category": "GROCERIES",
  "description": "Weekly groceries"
}
```

`description` is optional.

Allowed categories:

```text
PERSONAL
BUSINESS
PAYCHECK
TRAVEL
RENT
GROCERIES
```

Positive amounts can be used for income. Negative amounts can be used for expenses.

### List Transactions

```http
GET /transactions
```

Requires a bearer token.

Response:

```json
[
  {
    "amount": "25.5",
    "category": "GROCERIES",
    "description": "Weekly groceries",
    "created_at": "2026-06-07T12:00:00"
  }
]
```

### Get Balance

```http
GET /balance
```

Requires a bearer token.

Response:

```json
{
  "balance": "25.5"
}
```

## Local Setup

Create a `.env` file:

```env
DATABASE_URL=postgres://user:password@host:5432/finance
JWT_SECRET=finance-jwt-secret
HOST=0.0.0.0
PORT=7878
```

Run the SQLx migration:

```bash
cargo sqlx migrate run
```

Run the app:

```bash
cargo run
```

Test the health endpoint:

```bash
curl http://127.0.0.1:7878/health
```

## Docker

Build the image:

```bash
docker build -t finance_tracker .
```

Run the container:

```bash
docker run --env-file .env -e PORT=7878 -p 7878:7878 finance_tracker
```

Then visit:

```text
http://127.0.0.1:7878/health
```

## Example Flow

Create a user:

```bash
curl -X POST http://127.0.0.1:7878/users \
  -H "Content-Type: application/json" \
  -d '{"email":"finance@example.com","password":"example123"}'
```

Log in:

```bash
curl -X POST http://127.0.0.1:7878/login \
  -H "Content-Type: application/json" \
  -d '{"email":"finance@example.com","password":"example123"}'
```

Save the returned token, then add a transaction:

```bash
curl -X POST http://127.0.0.1:7878/transactions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{"amount":100.00,"category":"PAYCHECK","description":"Test paycheck"}'
```

Check the balance:

```bash
curl http://127.0.0.1:7878/balance \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## Notes

- Passwords are hashed with Argon2 before being stored.
- JWTs are used for protected routes.
- Each user only sees their own transactions.
- SQLx offline metadata is included so the Docker build does not need a database connection during compilation.
- Frontend still in progress
