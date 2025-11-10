import { Route, Routes } from 'react-router-dom';
import Homepage from './home/Homepage';
import Menu from './menu/Menu';
import Blog from './blog/Blog';

const Router = () => {
    return (
        <>
        <Menu /> {/**Menu above the Routes so it'll be shown on all subsites */}
        <Routes> {/**Show a specifict component based on the URL */}
            <Route path='/' element={<Homepage />} />
            <Route path='/blog' element={<Blog />} />
        </Routes>
        </>
    )
}

export default Router;