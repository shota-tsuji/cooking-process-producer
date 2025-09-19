FROM debian:bullseye

RUN apt update && apt install -y python3-dev python3-pip python3-venv curl

# Install uv
RUN pip3 install uv

COPY . .

# Install dependencies using uv
RUN uv pip install -r requirements.txt || uv pip install --system .

#RUN python3 -m grpc_tools.protoc -I./proto --python_out=. --pyi_out=. --grpc_python_out=. helloworld.proto

HEALTHCHECK CMD curl --fail http://localhost:8081/healthz || exit 1

#CMD ["uv", "run", "src/recipe.py"]
CMD ["uv", "run", "recipe.py"]
