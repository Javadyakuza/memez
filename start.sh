#!/bin/bash

# Database name (replace with your desired name)
DB_NAME="memez"

# Username for PostgreSQL access (replace with your username)
PG_USER="javadyakuza"

# Password for PostgreSQL access (replace with your password)
PG_PASSWORD="fuckon"

# Create the database with psql
psql postgres -U javadyakuza -c "CREATE DATABASE $DB_NAME OWNER $PG_USER;"

echo "Database $DB_NAME created successfully."
