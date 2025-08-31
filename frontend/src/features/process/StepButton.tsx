import {Button, Text} from '@mantine/core';

export const STEP_BUTTON_UNIT = {
    'height': 58,
};

export type StepButtonProps = {
    id: string;
    recipe_name: string;
    description: string;
    top: number;
    height: number;
};

export default function StepButton(props: StepButtonProps) {
    return (
        <Button className="process-step" style={{width: 250, height: props.height, top: props.top, left: 0}}>
            <Text>{props.recipe_name}:{props.description}</Text>
        </Button>
    );
}