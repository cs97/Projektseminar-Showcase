import { BrowserRouter } from 'react-router-dom';
import Router from './Router';

//App component is like the base of everything. I usually like to keep it as simple
//as possible but technically it doesnt matter if you start here with ur code
//sometimes for complex projects it can be practical to have this as a layer above all
//if you need to pass data between completely unrelated components
//you could also define some css basecode in the App.css file and integrate it in here
//so it will be passed down to every other component, which can be good for defining
//base colours, backgrounds, fonts, etc
const App = () => {
  return (
    <>
      <BrowserRouter> {/**Browser Router to show components based on the URL in the browser */}
        <Router /> {/**Router as extra component to keep it more simple and seperated */}
      </BrowserRouter>
    </>
  )
}

export default App;