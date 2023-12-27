### Setting up env
```env
RUST_LOG=cool_server

# DB
DB_URL=0.0.0.0:8000
DB_USER=<root_username>
DB_PASSWORD=<root_password>
DB_NS=example
DB_NAME=test
# DB

# SERVICE
SERVICE_HOST=0.0.0.0
SERVICE_PORT=3000
SERVICE_PWD_KEY=<password_key>
SERVICE_TOKEN_KEY=<token_key>
SERVICE_TOKEN_DURATION_SEC=1800
# SERVICE
```

### Run docker
`docker compose up -d`

### Start the server
`cargo watch -q -c -w src/ -x "run -- --no-capture"`
