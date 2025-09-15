## Install
```
uv sync
```
## How to Generate gRPC Code from .proto Files (Recommended)
```
❯ source .venv/bin/activate

❯ buf generate
Writing __init__.py
Writing proto/__init__.py
Writing proto/cooking/__init__.py
Writing proto/cooking/v1/__init__.py
```

Reference:
- [Update buf plugin with latest v2 beta · Issue #608 · danielgtaylor/python-betterproto](https://github.com/danielgtaylor/python-betterproto/issues/608#issuecomment-2599786298)
- [Buf + python-betterprotoプラグインを使ってgRPCサービスを実装する #Python - Qiita](https://qiita.com/inetcpl/items/925255d28cb11ebf3789)

## How to Generate gRPC Code from .proto Files
```
python -m grpc_tools.protoc -I./proto --python_out=. --pyi_out=. --grpc_python_out=. ./proto/helloworld.proto
```