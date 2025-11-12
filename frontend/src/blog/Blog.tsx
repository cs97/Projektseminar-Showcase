import './Blog.css'
import Blog_form from './Blog_form';
import Blog_timeline from './Blog_timeline';
import axios from 'axios';
import type { blog_post } from '../interface';

const Blog = () => {


    const timeline_content:{data: Array<blog_post>} = {
        data: []
    }

    //Serverdaten lesen
    function refresh () {
        (async () => {
            try {
                const {data} = await axios.get('/getall');
                timeline_content.data = data;
            } catch (error: unknown) {
                console.log(error);
                
            }
        })();
    }

    return (
        <div className='blog'>
            {/*Passing the refresh function down so it can be executed by lower components*/}
            <Blog_form refresh={refresh} />
            <Blog_timeline refresh={refresh} data={timeline_content.data} />
        </div>
    )
}

export default Blog;