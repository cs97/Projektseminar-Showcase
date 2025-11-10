import { useState } from 'react';
import type { ChangeEvent, FormEvent } from 'react'; //types imported extra
import axios from 'axios'; //axios module to for RESTful API operations
import './Blog_form.css' //css File with always same name as the component
import type { blog_post } from '../interface'; //import datatype from interface

//define a React function component
const Blog_form = ({refresh}: {refresh():void}) => {

    //statehook to managae the data of the user written post
    //const post = {title='', content:''}; would be simpler tho
    //the reason Ive still made a statehook even tho its more complicate is
    //statehooks will be used a lot more in bigger projects where data flows
    //from a parent to a child and backwards if theres not a seperate Redux system
    const [post, setPost] = useState({title : '', content: ''})

    //on keyboard input changes the variables behind he input fields accordingly
    function handleChange (event: ChangeEvent<HTMLInputElement> | ChangeEvent<HTMLTextAreaElement>) {
        setPost ((prevState: blog_post): blog_post => {
            return {...prevState, [event.target.name]: event.target.value}
        });
    }

    //executed when clicking submit button
    function handleSubmit(event: FormEvent) {
        const valid = true //could potentially be a form validation but is a constant here to save time
        event.preventDefault(); //prevents what an html form would normally do on submit
        //POST operation to /submit with the "post" object
        (async () => {
            try {
                if (valid) {
                    await axios.post('/submit', post);
                }else {
                    throw new Error("Ungültige Angaben");
                }
            } catch (error: unknown) {
                console.log(error);                
            }
        })();
        //auto refresh with a delay of 500ms after making a post
        setTimeout(() => {
            refresh()
        }, 500);   
    }

    return (
        <form className='form' onSubmit={handleSubmit}>
            <div className='input-box'>
                  <input
                      type='text'
                      id='title'
                      name='title'
                      placeholder='Titel'
                      value={post.title}
                      onChange={handleChange}
                  />
            </div>
            <div className='input-box'>
                  <textarea
                      id='content'
                      name='content'
                      placeholder='Beschreibung'
                      value={post.content}
                      onChange={(event) => {handleChange(event)}}
                  />
              </div>
            <div className='input-box'><button type='submit' className='btn'>Absenden</button></div>
        </form>
    )
}

export default Blog_form; //exports the component so it can be used inside others