import { useEffect } from 'react';
import './Blog_timeline.css'
import RefreshIcon from '@mui/icons-material/Refresh'; //RefreshIcon from googles mui-icons
import type { blog_post } from '../interface';

const Blog_timeline = ({refresh, data}: {refresh():void, data: Array<blog_post>}) => {

    //auto refresh the timeline component everytime its rendered
    useEffect (() => {
        refresh()
    });

    return (
        <>
        <div className="refresh" onClick={refresh}><RefreshIcon /></div>
        <div className='timeline'>
            {/**create a visual post for every post object inside the 'data' Array*/}
            {data.map((post) => {
                return (
                    <div className='post'>
                        <div className='post-title'>{post.title}</div>
                        <div className='post-content'>{post.content}</div>
                    </div>
                )
            })}
        </div>
        </>
    )
}

export default Blog_timeline;