# Handling databases
## Install diesel_cli 
    ```bash
    RUSTFLAGS="-L/opt/homebrew/opt/mysql-client/lib" cargo install diesel_cli --no-default-features --features mysql
    ```

## Make schema.rs
    
    ```bash
    diesel print-schema > src/infrastructure/database/schema.rs
    ```
