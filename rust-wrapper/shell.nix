{ pkgs ? import <nixpkgs> {} }:

let
  lib = pkgs.lib;
  merged-openssl = pkgs.symlinkJoin {
    name = "merged-openssl";
    paths = [ pkgs.openssl.out pkgs.openssl.dev ];
  };
in pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.cargo
    pkgs.pkg-config
    pkgs.openssl
    pkgs.sqlx-cli
    pkgs.postgresql
    pkgs.nats-server
    pkgs.natscli
  ];

  shellHook = ''
    export OPENSSL_DIR="${merged-openssl}"
    export PGDATA=./pgsql-data
    export PGHOST=/tmp
    export PGPORT=5432

    echo "Setting up PostgreSQL for trading-app..."

    if [ ! -d "$PGDATA" ]; then
      mkdir -p $PGDATA
      pg_ctl init -D $PGDATA
      echo "unix_socket_directories = '$PGHOST'" >> $PGDATA/postgresql.conf
      echo "logging_collector = off" >> $PGDATA/postgresql.conf
      echo "log_min_messages = fatal" >> $PGDATA/postgresql.conf
    fi

    pg_ctl start -D "$PGDATA" -l "$PGDATA/postgres.log" -o "-k $PGHOST" -w

    if ! psql -d postgres -tAc "SELECT 1 FROM pg_roles WHERE rolname='trading';" | grep -q 1; then
      echo "Creating role 'trading'..."
      psql -d postgres -c "CREATE ROLE trading LOGIN CREATEDB PASSWORD 'trading';"
    else
      echo "Role 'trading' already exists."
    fi

    if ! psql -lqt | cut -d \| -f 1 | grep -qw "tradingdb"; then
      echo "Creating database 'tradingdb'..."
      psql -d postgres -c "CREATE DATABASE tradingdb OWNER trading ENCODING 'UTF8';"
    else
      echo "Database 'tradingdb' already exists."
    fi

    export DATABASE_URL="postgresql://trading:trading@localhost:$PGPORT/tradingdb"

    if [ -d ./migrations ]; then
      sqlx database create
      sqlx migrate run
      echo "Database migrations have been applied."
    fi

    if ! pgrep -f nats-server >/dev/null; then
      echo "Starting NATS server on port 4222..."
      nats-server -p 4222 &
      export NATS_PID=$!
    fi

    function cleanup {
      echo "Stopping PostgreSQL..."
      pg_ctl stop -D $PGDATA > /dev/null 2>&1
      if [ -n "$NATS_PID" ]; then
        echo "Stopping NATS server..."
        kill $NATS_PID
      fi
    }
    trap cleanup EXIT

    echo "Trading environment is ready. Database URL: $DATABASE_URL"
  '';

  LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.openssl pkgs.libiconv ];
}
