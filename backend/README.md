# hubbit-backend

## sqlx

`cargo sqlx migrate run` to run migrations

`cargo sqlx migrate revert` to rollback migrations

`cargo sqlx prepare` to allow for offline compilation, such as in CI. Basically outputs a json with db meta data.
