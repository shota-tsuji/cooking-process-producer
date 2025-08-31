import React, {useMemo, useRef} from "react";
import {
    Recipe,
    useCreateProcessMutation,
    useCreateRecipeDetailMutation,
    useRecipesQuery
} from '../../generated/graphql';
import {ActionIcon, Box, Button, Paper, Title, Tooltip} from '@mantine/core';
import RecipeAddButton from "../../features/recipes/RecipeAddButton";
import {Link, useNavigate} from "react-router-dom";
import type {MRT_ColumnDef} from 'mantine-react-table';
import {MantineReactTable, MRT_TableInstance} from 'mantine-react-table';
import {IconEdit} from '@tabler/icons-react';
import {create} from "lodash";

export default function RecipeIndexPage() {
    const navigate = useNavigate();
    const [_createProcessResult, createProcess] = useCreateProcessMutation();
    const [result, _reexecuteQuery] = useRecipesQuery();
    const {data, fetching, error} = result;
    console.log(JSON.stringify({fetching, data, error}, null, 2));
    const tableInstanceRef = useRef<MRT_TableInstance<Recipe>>(null); //ts

    const columns = useMemo<MRT_ColumnDef<Recipe>[]>(
        () => [
            {
                accessorKey: 'title',
                header: 'Title',
            },
            {
                accessorKey: 'description',
                header: 'Description',
                mantineTableHeadCellProps: {sx: {color: 'green'}}, //custom props
            },
        ],
        [],
    );


    if (error != null) {
        return <>{JSON.stringify(error)}</>;
    }
    if (fetching) return <p>Loading...</p>;

    const recipes = data!.recipes.map((recipe) => (
        <tr key={recipe.id}>
            <td>
                <Link to={recipe.id}>{recipe.title}</Link>
            </td>
        </tr>
    ));

    const someEventHandler = () => {
        const rowSelection = tableInstanceRef.current.getState().rowSelection;
        console.info(rowSelection);
    };

    return (
        <React.Fragment>
            <Title order={1} mt="auto">Recipes</Title>
            <Paper shadow="xs" mt="md" p="lg">

                <RecipeAddButton/>
                <MantineReactTable
                    columns={columns}
                    data={data!.recipes}
                    enableRowActions
                    enableRowSelection //enable some features
                    enableColumnOrdering
                    enableGlobalFilter={false} //turn off a feature
                    getRowId={(originalRow) => originalRow.id}
                    renderTopToolbarCustomActions={({table}) => {
                        const handle = async () => {
                            const processInput: string[] = [];
                            console.info(table.getSelectedRowModel().flatRows);
                            table.getSelectedRowModel().flatRows.forEach(row => {
                                processInput.push(row.id);
                            })
                            console.info(processInput);

                            const result = await createProcess({ recipeIdList: {
                                recipeIdList : processInput
                                } });
                            const processId = result.data!.createProcess.id;
                            navigate(`/process/${processId}`);
                        };

                        return (
                            <div>
                                <Button onClick={handle}>
                                    {'Check process'}
                                </Button>
                            </div>
                        );
                    }}
                    renderRowActions={({row, table}) => (
                        <Box sx={{display: 'flex', gap: '16px'}}>
                            <Tooltip withArrow position="left" label="Edit">
                                <ActionIcon onClick={() => {
                                    console.info(row);
                                    navigate(`/recipes/${row.id}`)
                                    {/*
                                    navigate("/recipes/");
                                    open();
                                    setFormValue({id: row.id, name: row.name, amount: row.amount});
                                    */
                                    }
                                }}>
                                    <IconEdit/>
                                </ActionIcon>
                            </Tooltip>
                        </Box>
                    )}
                />
            </Paper>
        </React.Fragment>
    );
}
