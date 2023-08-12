# Handling databases
## Install diesel_cli 
    ```bash
    RUSTFLAGS="-L/opt/homebrew/opt/mysql-client/lib" cargo install diesel_cli --no-default-features --features mysql
    ```

## Make schema.rs
    
    ```bash
    diesel print-schema > src/infrastructure/database/schema.rs
    ```

## Init first migration
    ```bash
    diesel migration generate create_table #done
    mysqldump -h127.0.0.1 -uroot -P3307 -p --column-statistics=0 --no-data poolweb > migrations/2023-08-10-160702_initial_migration/up.sql
    ```

## Testing
    ### Set up test database
        ```bash
        docker pull arm64v8/mariadb
        docker run -p 3399:3306 --name poolweb-csv-extractor-mariadb --env MARIADB_USER=test --env MARIADB_PASSWORD=test --env MARIADB_ROOT_PASSWORD=test  --env MARIADB_DATABASE=test arm64v8/mariadb:latest
        ```
    ### Run test
        ```bash
        cargo test -- --test-threads=1
        ```

