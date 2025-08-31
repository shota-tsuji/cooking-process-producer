import React from "react";
import {useParams} from "react-router-dom";
import {useResourceQuery} from "../../generated/graphql";
import {Center, Text, Title} from "@mantine/core";

export default function ResourceDetailPage() {
    const params = useParams();
    const [result, _reexecuteQuery] = useResourceQuery({variables: {resourceId: params.resourceId!}});
    const {data, fetching, error} = result;
    console.info(data);

    if (error != null) {
        return <>{JSON.stringify(error)}</>;
    }
    if (fetching || data == null) return <p>Loading...</p>;

    const {id, name, amount} = data.resource;

    return (
        <React.Fragment>
            <Title order={1} mt="auto">{name}</Title>
            <Center>
                <Text>Amount: {amount}</Text>
            </Center>
        </React.Fragment>
    );
}