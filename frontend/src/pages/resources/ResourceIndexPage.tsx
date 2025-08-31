import React, {useState} from "react";
import {Link} from "react-router-dom";
import {Resource, useResourcesQuery} from "../../generated/graphql";
import {Box, Button, Dialog, Group, NumberInput, Paper, Table, TextInput, Title} from "@mantine/core";
import type {MRT_ColumnDef} from 'mantine-react-table';
import {useForm} from "@mantine/form";
import { useNavigate } from "react-router-dom";

export default function ResourceIndexPage() {
    const navigate = useNavigate();
    const [result, _reexecuteQuery] = useResourcesQuery();
    const {data, fetching, error} = result;
    console.log(JSON.stringify({fetching, data, error}, null, 2));

    if (error != null) {
        return <>{JSON.stringify(error)}</>;
    }
    if (fetching) return <p>Loading...</p>;

    const resourceList = data!.resources.map(resource => (
        <tr key={resource.id}>
            <td>
                <Link to={resource.id.toString()}>{resource.name}</Link>
            </td>
            <td>{resource.amount}</td>
        </tr>
    ));

    return (
        <React.Fragment>
            <Title order={1} mt="auto">Resources</Title>
            <Button onClick={() => {navigate(`/resources/new`);}}>Create</Button>
            <Paper shadow="xs" mt="md" p="lg">
                <Table highlightOnHover>
                    <thead>
                    <tr>
                        <th>Name</th>
                        <th>Amount</th>
                    </tr>
                    </thead>
                    <tbody>{resourceList}</tbody>
                </Table>
            </Paper>
        </React.Fragment>
    );
}

interface EditResourceDialogProps {
    columns: MRT_ColumnDef<Resource>[];
    onClose: () => void;
    onSubmit: (values: Resource) => void;
    open: boolean;
    formValue: Resource;
}

export const EditResourceDialog = ({columns, onClose, onSubmit, open, formValue}: EditResourceDialogProps) => {
    const [values, setValues] = useState<any>(() =>
        columns.reduce((acc, column) => {
            acc[column.accessorKey ?? ''] = '';
            return acc;
        }, {} as any),
    );
    const form = useForm({
        initialValues: {
            name: formValue.name,
            amount: formValue.amount,
        }
    });
    const handleSubmit = () => {
        //put your validation logic here
        onSubmit(values);
        onClose();
    };

    return (
        <Box maw={300} mx="auto">
            <Dialog opened={open}>
                <Title ta="center">Resource: {formValue.id}</Title>
                <form onSubmit={handleSubmit}>
                    <TextInput
                        withAsterisk
                        label="Name"
                        placeholder="your@email.com"
                        {...form.getInputProps('name')}
                    />

                    <NumberInput
                        withAsterisk
                        mt="md"
                        label="Amount"
                        {...form.getInputProps('amount')}
                    />

                    <Group position="right" mt="md">
                        <Button type="submit">Submit</Button>
                    </Group>
                </form>
            </Dialog>
        </Box>
    );
}

type ResourceEditProps = {
    id: number;
    name: string;
    amount: number;
}

function ResourceEdit({id, name, amount}: ResourceEditProps) {
    const form = useForm({
        initialValues: {
            name: name,
            amount: amount,
        }
    });
    return (
        <Box maw={300} mx="auto">
            <form onSubmit={(e) => e.preventDefault()}>
                <TextInput
                    withAsterisk
                    label="Name"
                    placeholder="your@email.com"
                    {...form.getInputProps('name')}
                />

                <NumberInput
                    withAsterisk
                    mt="md"
                    label="Amount"
                    {...form.getInputProps('amount')}
                />

                <Group position="right" mt="md">
                    <Button type="submit">Submit</Button>
                </Group>
            </form>
        </Box>
    );
}
