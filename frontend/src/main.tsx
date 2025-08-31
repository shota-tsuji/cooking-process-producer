import ReactDOM from 'react-dom/client'
import { createClient, Provider, cacheExchange, fetchExchange } from "urql";
import App from './App.tsx'
import './index.css'

const client = createClient({
  url: 'http://0.0.0.0:8080/',
  exchanges: [cacheExchange, fetchExchange]
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    // comment out to test react-beautiful-dnd on development mode
    // https://github.com/atlassian/react-beautiful-dnd/issues/2396
  //<React.StrictMode>
    <Provider value={client}>
        <App />
    </Provider>
  //</React.StrictMode>,
)
