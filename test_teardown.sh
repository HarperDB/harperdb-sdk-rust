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

# curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
# --header 'Content-Type: application/json' \
# --data-raw '{
#   "operation":"create_schema",
#   "schema": "schema_test"
# }'


# curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
# --header 'Content-Type: application/json' \
# --data-raw '{
# "operation":"drop_table",
# "schema":"shop",
# "table":"product"
# }'

# curl --location --request POST 'http://localhost:9925' \
# --header 'Content-Type: application/json' \
# --header 'Authorization: Basic SERCX0FETUlOOnBhc3N3b3Jk' \
# --data-raw '{
# "operation":"drop_attribute",
# "schema":"testing",
# "table":"drop_attribute_table_test",
# "attribute":"breed"
# }'