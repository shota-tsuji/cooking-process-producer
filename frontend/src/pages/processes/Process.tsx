import {useParams} from "react-router-dom";
import StepButton, {STEP_BUTTON_UNIT, StepButtonProps} from "../../features/process/StepButton";
import {useProcessQuery, useResourcesQuery} from "../../generated/graphql";

type ResourceColumnProps = {
    top: number;
    left: number;
    height: number;
    steps: StepButtonProps[];
}

function ResourceColumn(props: ResourceColumnProps) {
    const step_buttons = props.steps.map((step) => <StepButton id={step.id} recipe_name={step.recipe_name}
                                                               top={step.top} height={step.height}
                                                               description={step.description}
                                                               key={step.id}/>);
    return (
        <div className="process-step" style={{top: props.top, left: props.left, height: props.height, width: 260}}>
            {step_buttons}
        </div>
    );
}

type VerticalStartBarProps = { height: number; width: number; }

function VerticalStartBar(props: VerticalStartBarProps) {
    return (
        <div className="process-bar-vertical" style={{top: 0, left: 0, height: props.height, width: props.width}}>
        </div>
    );
}

type TimeLabelProps = {
    time: number;
}

function TimeLabel(props: TimeLabelProps) {
    return (
        <div style={{top: 0, left: 0, width: 80, height: 60}}>
            <span>{props.time}min</span>
        </div>
    );
}

type TimeHorizontalBarProps = {
    width: number;
    height: number;
    top: number;
}

function TimeHorizontalBar(props: TimeHorizontalBarProps) {
    return (
        <div className="div-horizontal" style={{top: props.top, height: props.height, width: props.width}}>
        </div>
    );
}

export const PROCESS_GRID_UNIT = {
    'height': 60,
    'width': 260,
    'left_pad': 30,
    'unit_of_time': 5,
};

export function ProcessGrid() {
    const height = 60;
    const width = 260;
    const left_pad = 30;
    const unit_of_time = 5;

    const {processId} = useParams();
    const [processResult, _recipeReexecuteQuery] = useProcessQuery({variables: {processId: processId!}});
    const [resourceResult, _reexecuteQuery] = useResourcesQuery();

    if (processResult.error != null || resourceResult.error != null) {
        return <>{JSON.stringify(processResult.error)}</>;
    }

    if (processResult.fetching || processResult.data == null) {
        return <p>recipe Loading...</p>;
    }

    const resourceInfos = processResult.data!.process.resourceInfos
    console.info(processResult.data!.process.resourceInfos);

    if (resourceResult.fetching || resourceResult.data == null) {
        return <p>resource Loading...</p>;
    }

    const step_lists: StepButtonProps[][] = [];
    resourceInfos.forEach(resourceInfo => {
        const steps: StepButtonProps[] = [];
        resourceInfo.steps.forEach(step => {
            const s: StepButtonProps = {
                id: step.id,
                recipe_name: step.recipeName,
                description: step.description,
                top: (step.startTime / PROCESS_GRID_UNIT.unit_of_time) * PROCESS_GRID_UNIT.height,
                height: (step.duration / PROCESS_GRID_UNIT.unit_of_time - 1) * PROCESS_GRID_UNIT.height + STEP_BUTTON_UNIT.height,
            }
            steps.push(s)
        });
        step_lists.push(steps);
    });
    console.info("step_lists");
    console.info(step_lists);

    const resource_kind = step_lists.length;
    // TODO: set from max of end time
    const time_label_count = 12;
    const time_label_values = Array.from(Array(time_label_count).keys()).map(x => x * unit_of_time);
    const time_label_list = time_label_values.map((value) => <TimeLabel time={value}/>);

    const bar_width = width * resource_kind;
    const bar_list = Array.from(Array(time_label_count).keys())
        .map(i => <TimeHorizontalBar top={i * height}
                                     height={height}
                                     width={bar_width}/>);
    const bar_height = (time_label_count - 1) * height;

    const resource_columns = Array.from(Array(resource_kind).keys())
        .map(i => <ResourceColumn top={0}
                                  left={i * width + left_pad}
                                  height={bar_height}
                                  steps={step_lists[i]}/>);

    return (
        <div>
            <h1>Recipes Process</h1>
            <div className="recipe-grid">
                <div className="process-bar-side">
                    {time_label_list}
                </div>
                <div className="process" style={{left: 80}}>
                    <div className="process-bar-horizontal" style={{left: left_pad}}>
                        {bar_list}
                    </div>

                    <VerticalStartBar height={bar_height} width={left_pad}/>

                    {resource_columns}
                </div>
            </div>
        </div>
    );
}
