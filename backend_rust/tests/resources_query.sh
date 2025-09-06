curl -X POST http://localhost:8080 \
  -H "Content-Type: application/json" \
  -d '{"query":"query resources { resources { id name amount } }"}' | jq