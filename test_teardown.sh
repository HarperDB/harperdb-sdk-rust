curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_schema",
  "schema": "create_schema_test"
}'


curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_schema",
  "schema": "drop_schema_test"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_schema",
  "schema": "testing"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_schema",
  "schema": "describe_schema_test"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_user",
  "username": "created_hdb_user"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_user",
  "username": "alter_hdb_user"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"drop_user",
  "username": "delete_hdb_user"
}'
