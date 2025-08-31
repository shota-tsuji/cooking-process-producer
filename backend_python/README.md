## Install
```
uv sync
```

## How to Generate gRPC Code from .proto Files
```
python -m grpc_tools.protoc -I./proto --python_out=. --pyi_out=. --grpc_python_out=. ./proto/helloworld.proto
```