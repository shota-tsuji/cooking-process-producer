import React from "react";
import {Route, Routes} from "react-router-dom";
import RecipeIndexPage from "./RecipeIndexPage";
import RecipeDetailPage from "./recipeDetailPage";
import RecipeDetailEdit from "../../features/recipes/RecipeDetailEdit";

// use index route to distinguish index from matching other patterns.
// https://reactrouter.com/en/main/route/route#index
export default function RecipeRouter() {
    return (
        <React.Fragment>
            <div>
                <Routes>
                    <Route path="/" element={<RecipeIndexPage/>}/>
                    <Route path=":recipeId/" element={<RecipeDetailPage/>}/>
                    <Route path=":recipeId/edit" element={<RecipeDetailEdit/>}/>
                </Routes>
            </div>
        </React.Fragment>
    );
}
