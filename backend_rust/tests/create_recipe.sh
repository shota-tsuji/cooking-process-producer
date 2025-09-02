curl 'http://localhost:8080' \
  -X POST \
  -H 'content-type: application/json' \
  --data '{
    "query": "mutation { createRecipeDetail(recipeDetailData: { title: \"recip0\", description: \"a\", steps: [{ description: \"a\", resourceId: 1, orderNumber: 0, duration: 10 }] }) { id title description steps { id description resourceId orderNumber duration } } }"
}'
