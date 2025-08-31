import {useNavigate, useParams} from "react-router-dom";
import {
    CreateStepInput,
    RecipeDetail, Resource, Step,
    StepInput,
    useCreateStepMutation,
    useRecipeDetailQuery, useResourcesQuery,
    useUpdateRecipeDetailMutation
} from "../../generated/graphql";
import {useForm} from "@mantine/form";
import {Box, Button, Center, Code, Group, NumberInput, Select, Text, TextInput, Title} from "@mantine/core";
import {DragDropContext, Draggable, Droppable} from 'react-beautiful-dnd';
import {IconGripVertical} from '@tabler/icons-react';
import * as _ from 'lodash';

// TODO: order_number 更新だけをする form 作成
export default function RecipeDetailEdit() {
    const {recipeId} = useParams();
    const [recipeDetailResult, _recipeDetailReexecuteQuery] = useRecipeDetailQuery({variables: {recipeId: recipeId!}});
    const [resourceResult, _resourceReexecuteQuery] = useResourcesQuery();

    if (recipeDetailResult.error != null || resourceResult.error != null) {
        return <>{JSON.stringify(recipeDetailResult.error)}</>;
    }

    if (recipeDetailResult.fetching || recipeDetailResult.data == null) {
        return <p>recipe Loading...</p>;
    }
    if (resourceResult.fetching || resourceResult.data == null) {
        return <p>recipe Loading...</p>;
    }

    const {id, title, description, steps} = recipeDetailResult.data!.recipeDetail;
    const resources = resourceResult.data!.resources;

    return (
        <RecipeEdit description={description} id={id} steps={steps} title={title} resources={resources}/>
    );
}

type RecipeEditProps = {
   id: string;
   title: string;
   description: string;
   steps: Step[];
   resources: Resource[];
}

function RecipeEdit({id, title, description, steps, resources}: RecipeEditProps) {
    const navigate = useNavigate();
    const [_updateRecipeDetailResult, updateRecipeDetail] = useUpdateRecipeDetailMutation();
    const [_createStepResult, createStep] = useCreateStepMutation();

    // Prepare resource select data
    const resourceData = resources.map(resource => {
        return { value: resource.id, label: resource.name }
    });

    // To compare modified steps, keep original recipe detail
    const originalSteps = _.cloneDeep(steps);

    const form = useForm({
        initialValues: {
            title: title,
            description: description,
            steps: steps,
            resourceId: 1,
        },
    });
    /*
                    <NumberInput
                        withAsterisk
                        mt="md"
                        label="Resource"
                        {...form.getInputProps(`steps.${index}.resourceId`)}
                    />
     */

    const fields = form.values.steps.map((_, index) => (
        <Draggable key={index} index={index} draggableId={index.toString()}>
            {(provided) => (
                <Group ref={provided.innerRef} mt="xs" {...provided.draggableProps}>
                    <Center {...provided.dragHandleProps}>
                        <IconGripVertical size="1.2rem"/>
                    </Center>
                    <TextInput
                        withAsterisk
                        mt="md"
                        label="Title"
                        placeholder="John Doe"
                        {...form.getInputProps(`steps.${index}.description`)}
                    />
                    <NumberInput
                        withAsterisk
                        mt="md"
                        label="Duration"
                        {...form.getInputProps(`steps.${index}.duration`)}
                    />
                    <Select
                        mt="md"
                        label="Resource"
                        data={resourceData}
                        {...form.getInputProps(`steps.${index}.resourceId`)}
                    />
                </Group>
            )}
        </Draggable>
    ));

    // TODO: Filtering with respect to step ids
    const handleSubmit = async (values: RecipeDetail) => {
        //console.info(originalSteps);
        //console.info(values);

        const createdSteps: CreateStepInput[] = [];
        /*
        values.steps.map((s, index) => {
            // update order number as permuted order in the form
            s.orderNumber = index;
            const stepInput: CreateStepInput = {id: s.id, description: s.description, duration: s.duration, orderNumber: index, resourceId: s.resourceId};
            return stepInput;
        }).filter(step => step.id == '');
         */
        values.steps.forEach((step, index) => {
            if (step.id == '') {
                const createdStep: CreateStepInput = {
                    description: step.description,
                    duration: step.duration,
                    orderNumber: index,
                    resourceId: step.resourceId
                }
                createdSteps.push(createdStep);
            }
        });

        // Filtering
        // Loop for values and check each step_id and update (set) orderNumber
        const updatedSteps = values.steps.map((s, index) => {
            // update order number as permuted order in the form
            s.orderNumber = index;
            //console.info(s, index);
            const stepInput: StepInput = {id: s.id, description: s.description, duration: s.duration, orderNumber: index, resourceId: s.resourceId};
            return stepInput;
        }).filter(step => step.id != '');
        console.info(createdSteps);
        await updateRecipeDetail({recipeDetailData: {id: id, title: values.title, description: values.description, steps: updatedSteps}});
        await createStep({createRecipeStepData: {id: id, steps: createdSteps}});

        // If id is empty, its step is treated as creation
        navigate(`/recipes/${id}`);
    };

    // TODO: validate description, resourceId, duration
    return (
        <Box maw={800} mx="auto">
            <Title order={1} mt="auto">Edit Recipe</Title>
            <form onSubmit={form.onSubmit(async (values) => {
                    const recipeDetail = {id: id, title: values.title, description: values.description, steps: values.steps};
                    await handleSubmit(recipeDetail);
                }
            )}>
                <TextInput
                    placeholder="example@mail.com"
                    {...form.getInputProps(`title`)}
                />
                <TextInput
                    placeholder="example@mail.com"
                    {...form.getInputProps(`description`)}
                />

                    <DragDropContext
                        onDragEnd={({destination, source}) => {
                            form.reorderListItem('steps', {from: source.index, to: destination!.index});
                            //console.info(form.values.steps);
                        }
                        }
                    >
                        <Droppable droppableId="dnd-list" direction="vertical">
                            {(provided) => (
                                <div {...provided.droppableProps} ref={provided.innerRef}>
                                    {fields}
                                    {provided.placeholder}
                                </div>
                            )}
                        </Droppable>
                    </DragDropContext>

                <Group position="right" mt="md">
                    <Button type="submit">Submit</Button>
                </Group>
            </form>

            <Group position="center" mt="md">
                <Button onClick={() => form.insertListItem('steps', {id: '', description: '', resourceId: '', orderNumber: '', duration: '', })}>
                    Add cooking step
                </Button>
            </Group>

            <Text size="sm" weight={500} mt="md">
                Form values:
            </Text>
            <Code block>{JSON.stringify(originalSteps, null, 2)}</Code>
        </Box>
    );
}