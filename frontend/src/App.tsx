import './App.css'

import { MantineProvider } from '@mantine/core';
import { BrowserRouter as Router } from "react-router-dom";
import MainRouter from './pages/MainRoutes';

function App() {
  //const [result, _reexecuteQuery] = useRecipeDetailQuery({ variables: { recipeId: "00000000-0000-0000-0000-000000000000" } })

  //let recipeDetail = data!.recipeDetail;
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  //let {id, title, description, steps, ...rest } = recipeDetail;

  return (
<>
<MantineProvider withGlobalStyles withNormalizeCSS>
<Router>
<MainRouter />
</Router>
</MantineProvider>

</>
  )
}

export default App
