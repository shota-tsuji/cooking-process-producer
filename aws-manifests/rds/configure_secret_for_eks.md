```
aws iam create-policy --policy-name kms_read_policy --policy-document file://aws-manifests/kms/kms_read_policy.json
aws iam create-policy --policy-name rds_password_read_policy --policy-document file://aws-manifests/rds/rds_read_policy.json

AWS_PROFILE=eks-admin eksctl create iamserviceaccount --name cooking-api-sa --namespace dev-cooking-api --cluster process-pj --role-name cooking_deploy_role_dev --attach-policy-arn arn:aws:iam::763348989563:policy/rds_password_read_policy --approve --verbose 4
```
