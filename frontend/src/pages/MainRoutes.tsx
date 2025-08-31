import React from "react";
import {Route, Routes} from "react-router-dom";
import RecipeRouter from "./recipes/RecipeRoutes";
import ResourceRouter from "./resources/ResourceRoutes";
import {ProcessGrid} from "./processes/Process";

// use wildcard pattern to segregate and match to multiple nested routes.
// https://reactrouter.com/en/main/route/route#splats
// https://stackoverflow.com/questions/70815495/react-nested-routes-for-every-folder
export default function MainRouter() {
    return (
        <React.Fragment>
            <Routes>
                <Route path="/recipes/*" element={<RecipeRouter/>}/>
                <Route path="/resources/*" element={<ResourceRouter/>}/>
                <Route path="/process/:processId" element={<ProcessGrid/>}/>
            </Routes>
        </React.Fragment>
    );
}
