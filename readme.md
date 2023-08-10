# Handling databases
## Install diesel_cli 
    ```bash
    RUSTFLAGS="-L/opt/homebrew/opt/mysql-client/lib" cargo install diesel_cli --no-default-features --features mysql
    ```

## Make schema.rs
    
    ```bash
    diesel print-schema > src/infrastructure/database/schema.rs
    ```

## Testing
    ```bash
    docker pull arm64v8/mariadb
    docker run -p 3399:3306 --name poolweb-csv-extractor-mariadb --env MARIADB_USER=test --env MARIADB_PASSWORD=test --env MARIADB_ROOT_PASSWORD=test  --env MARIADB_DATABASE=test arm64v8/mariadb:latest
    ```