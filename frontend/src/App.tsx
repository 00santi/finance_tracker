import React, {useState} from 'react';
import MessageDisplay from './MessageDisplay.tsx';

//import './App.css';

function App() {
    const [message, setMessage] = useState('Welcome to the Finance Tracker App');

    const [_, setClicks] = useState(0);

    const handleCounterClick = () => {
        setClicks(prev => {
            const newClicks = prev + 1;
            if (newClicks === 1)
                setMessage('Button Clicked');
            else
                setMessage(`Button Clicked ${newClicks} times`);

            return newClicks;
        })
    };

    const [inputText, setInputText] = useState('');
    const [responseText, setResponseText] = useState('');

    const handleInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        setInputText(event.target.value);
    }

    const handleSubmit = () => {
        setResponseText(inputText);
        setInputText('');
    };


    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <MessageDisplay message={message}></MessageDisplay>
                <button onClick={handleCounterClick}>Change Message</button>

                <hr style={{margin: '20px 0'}}/>

                <h2>User Input Example</h2>
                <input type="text"
                       value={inputText}
                       onChange={handleInput}
                       placeholder="Enter email here"/>
                <button onClick={handleSubmit}>Submit</button>
                {responseText && (
                    <p>You submitted: <strong>{responseText}</strong></p>)}
            </header>
        </div>
    )
}

export default App;
