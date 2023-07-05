$ErrorActionPreference = "Stop"

if (-not (Test-Path $(Get-Command -Name "psql" -ErrorAction SilentlyContinue).Source)) {
  Write-Host "Error: psql is not installed." -ForegroundColor Red
  exit 1
}

if (-not (Test-Path $(Get-Command -Name "sqlx" -ErrorAction SilentlyContinue).Source)) {
  Write-Host "Error: sql is not installed." -ForegroundColor Red
  Write-Host "Use:" -ForegroundColor Red
  Write-Host "    cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres" -ForegroundColor Red
  Write-Host "to install it" -ForegroundColor Red
  exit 1
}

# Check if a custom user has been set, otherwise default to 'postgres'
$DB_USER = $env:POSTGRES_USER
if ([string]::IsNullOrWhiteSpace($DB_USER)) {
  $DB_USER = "postgres"
}

# Check if a custom password has been set, otherwise default to 'password'
$DB_PASSWORD = $env:POSTGRES_PASSWORD
if ([string]::IsNullOrWhiteSpace($DB_PASSWORD)) {
  $DB_PASSWORD = "password"
}

# Check if a custom database name has been set, otherwise default to 'newsletter'
$DB_NAME = $env:POSTGRES_DB
if ([string]::IsNullOrWhiteSpace($DB_NAME)) {
  $DB_NAME = "newsletter"
}

# Check if a custom port has been set, otherwise default to '5432'
$DB_PORT = $env:DB_PORT
if ([string]::IsNullOrWhiteSpace($DB_PORT)) {
  $DB_PORT = "5432"
}

# Launch postgres using Docker
if (-not [string]::IsNullOrWhiteSpace($env:SKIP_DOCKER)) {
  $env:POSTGRES_USER = $DB_USER
  $env:POSTGRES_PASSWORD = $DB_PASSWORD
  $env:POSTGRES_DB = $DB_NAME
  $env:DB_PORT = $DB_PORT

  docker run `
    -e POSTGRES_USER=$DB_USER `
    -e POSTGRES_PASSWORD=$DB_PASSWORD `
    -e POSTGRES_DB=$DB_NAME `
    -p $DB_PORT:5432 `
    -d postgres `
    postgres -N 1000
    # ^ Increased maximum number of connection for testing purposes
}

# Keep pinging Postgres until it's ready to accept commands
$env:PGPASSWORD = $DB_PASSWORD

while (-not (& psql -h "localhost" -U $DB_USER -p $DB_PORT -d "postgres" -c '\q' -ErrorAction SilentlyContinue)) {
  Write-Host "Postgres is still unavailable - sleeping" -ForegroundColor Yellow
  Start-Sleep -Seconds 1
}

Write-Host "Postgres is up and running on port $DB_PORT" -ForegroundColor Green

$env:DATABASE_URL = "postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
sqlx database create
# sqlx migrate add create_subscriptions_table
sqlx migrate run

Write-Host "Postgres has been migrated, ready to go!" -ForegroundColor Green
