import './Menu.css'
import { Link } from 'react-router-dom';

const Menu = () => {
    return (
        <div className='menu'>
            {/**always use those Link tags from react instead of <a href=...> */}
            <Link to={'/'}><div className='menu-item'>Home</div></Link>
            <Link to={'/blog'}><div className='menu-item'>Blog</div></Link>
            <div className='menu-item'>Dummy3</div>
            <div className='menu-item'>Dummy4</div>
        </div>
    )
}

export default Menu;