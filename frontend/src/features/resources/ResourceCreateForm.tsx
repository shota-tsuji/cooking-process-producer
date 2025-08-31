import {useCreateResourceMutation} from "../../generated/graphql";
import {useForm} from "@mantine/form";
import {Box, Button, Group, NumberInput, TextInput, Title} from "@mantine/core";
import { useNavigate } from "react-router-dom";

export default function ResourceCreateForm() {
    const navigate = useNavigate();
    const [_createResourceResult, createResource] = useCreateResourceMutation();

    const form = useForm({
        initialValues: {
            name: "",
            amount: 1,
        }
    });

    const handleSubmit = async (values: { name: string; amount: number; }) => {
        console.info(values);
        await createResource({resourceData: {name: values.name, amount: values.amount}});
        navigate(`/resources/`);
    };

    return (
        <Box maw={300} mx="auto">
            <Title order={1} mt="auto">New Resource</Title>
            <form onSubmit={form.onSubmit(async (values) => {
                    await handleSubmit({name: values.name, amount: values.amount});
                }
            )}>
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