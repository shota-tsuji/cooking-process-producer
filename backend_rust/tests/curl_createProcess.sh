#!/bin/bash

## Update the URL to your GraphQL endpoint
#GRAPHQL_ENDPOINT="http://localhost:8080/"
#
## Example recipeIdList payload
#mutation='mutation createProcess(recipeIdList: CreateProcessInput!) { createProcess(recipeIdList: { "recipeIdList": { "recipeIdList": ["recipe-id-1", "recipe-id-2"] } }) { id } }'
#
#curl -X POST "$GRAPHQL_ENDPOINT" \
#  -H "Content-Type: application/json" \
#  -d "{\"query\": \"$mutation}"


#!/bin/bash
GRAPHQL_ENDPOINT="http://localhost:8080/"

curl -X POST "$GRAPHQL_ENDPOINT" \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation createProcess($recipeIdList: CreateProcessInput!) { createProcess(recipeIdList: $recipeIdList) { id } }",
    "variables": { "recipeIdList": { "recipeIdList": ["01K5115CRCFR5YRNZ0HX9Y6J7J", "01K5119WR05236RWT52HTD22ZQ"] } }
  }'