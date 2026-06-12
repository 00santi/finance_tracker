import React, {useState} from 'react';

//import './App.css';

function App() {
    const [message, setMessage] = useState('Welcome to the Finance Tracker App');

    const [_, setClicks] = useState(0);

    const handleClick = () => {
        setClicks(prev => {
            const newClicks = prev + 1;
            if (newClicks === 1)
                setMessage('Button Clicked');
            else
                setMessage(`Button Clicked ${newClicks} times`);

            return newClicks;
        })
    };
    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <p>{message}</p>
                <button onClick={handleClick}>Change Message</button>
            </header>
        </div>
    )
}

export default App;
