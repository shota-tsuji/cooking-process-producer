pub mod process_service_client;
pub mod recipe_mapper;
pub mod resource_mapper;
pub mod step_mapper;

// Define the gRPC modules once here to avoid type mismatches
pub mod cooking {
    tonic::include_proto!("proto.cooking.v1");
}
