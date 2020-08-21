curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_schema",
  "schema": "describe_schema_test"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_schema",
  "schema": "drop_schema_test"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_schema",
  "schema": "testing"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_table",
  "schema": "testing",
  "table": "drop_table_test",
  "hash_attribute": "id"
}'

curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_table",
  "schema": "testing",
  "table": "describe_table_test",
  "hash_attribute": "id"
}'


# drop_attribute
##########################
curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_table",
  "schema": "testing",
  "table": "drop_attribute_table_test",
  "hash_attribute": "id"
}'

sleep 1

curl --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--header 'Authorization: Basic SERCX0FETUlOOnBhc3N3b3Jk' \
--data-raw '{
"operation":"insert",
"schema":"testing",
"table":"drop_attribute_table_test",
"records": [
  {
    "name":"Harper",
    "breed":"Mutt",
    "id":"1",
    "age":5    
  },
  {
    "name":"Penny",
    "breed":"Mutt",
    "id":"3",
    "age":5    
  }
]
}'


# NOSQL CRUD Test
##########################
curl -u HDB_ADMIN:password --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--data-raw '{
  "operation":"create_table",
  "schema": "testing",
  "table": "crud_table_test",
  "hash_attribute": "id"
}'

curl --location --request POST 'http://localhost:9925' \
--header 'Content-Type: application/json' \
--header 'Authorization: Basic SERCX0FETUlOOnBhc3N3b3Jk' \
--data-raw '{
"operation":"insert",
"schema":"testing",
"table":"crud_table_test",
"records": [
  {
    "name":"Harper",
    "breed":"Mutt",
    "id":"updaterecord1234",
    "age":5    
  },
  {
    "name":"Penny",
    "breed":"Mutt",
    "id":"deleterecord1234",
    "age":5    
  },
  {
    "name":"Tom Ford",
    "breed":"Pug",
    "id":"searchbyhash1234",
    "age":1   
  }
]
}'



# ADD USERs
##########################
curl --location --request POST 'http://localhost:9925?=' \
--header 'Content-Type: application/json' \
--header 'Authorization: Basic SERCX0FETUlOOnBhc3N3b3Jk' \
--data-raw '{
	"operation":"add_user",
	"role":"c0a90733-1fc3-48df-a16b-d7c3011b63b2",
	"username":"alter_hdb_user",
	"password":"password", 
	"active":true
}'

curl --location --request POST 'http://localhost:9925?=' \
--header 'Content-Type: application/json' \
--header 'Authorization: Basic SERCX0FETUlOOnBhc3N3b3Jk' \
--data-raw '{
	"operation":"add_user",
	"role":"c0a90733-1fc3-48df-a16b-d7c3011b63b2",
	"username":"delete_hdb_user",
	"password":"password", 
	"active":true
}'