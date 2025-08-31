import React from "react";
import {Route, Routes} from "react-router-dom";
import ResourceIndexPage from "./ResourceIndexPage";
import ResourceDetailPage from "./ResourceDetailEdit";
import ResourceDetailEdit from "./ResourceDetailEdit";
import ResourceCreateForm from "../../features/resources/ResourceCreateForm";

export default function ResourceRouter() {
    return (
        <React.Fragment>
            <Routes>
                <Route index element={<ResourceIndexPage/>}/>
                <Route path=":resourceId/" element={<ResourceDetailPage/>}/>
                <Route path=":resourceId/edit" element={<ResourceDetailEdit/>}/>
                <Route path="new" element={<ResourceCreateForm/>}/>
            </Routes>
        </React.Fragment>
    );
}
