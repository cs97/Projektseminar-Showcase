import './Blog.css'
import Blog_form from './Blog_form';
import Blog_timeline from './Blog_timeline';
import axios from 'axios';
import type { blog_post } from '../interface';
import { useEffect, useState } from 'react';

const Blog = () => {

    const [userData, setUserData] = useState<Array<blog_post>>([])

    //Serverdaten lesen
    function refresh () {
        (async () => {
            try {
                const {data} = await axios.get('/getall');
                setUserData(data.arr||[])
            } catch (error) {
                console.log("failed to fetch data:", error);
                setUserData([])
            }
        })();
    }        

    useEffect(() => {
        refresh()
    },[]);

    return (
        <div className='blog'>
            {/*Passing the refresh function down so it can be executed by lower components*/}
            <Blog_form refresh={refresh} />
            <Blog_timeline refresh={refresh} data={userData} />
        </div>
    )
}

export default Blog;