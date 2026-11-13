#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 \
    --username "$POSTGRES_USER" \
    --dbname "$POSTGRES_DB" <<-EOSQL

    CREATE DATABASE phantom0_users;
    CREATE DATABASE phantom0_identity;
    CREATE DATABASE phantom0_groups;
    CREATE DATABASE phantom0_notifications;

EOSQL
