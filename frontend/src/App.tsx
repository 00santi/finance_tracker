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

    const [emailInput, setEmailInput] = useState('');
    const [finalEmail, setFinalEmail] = useState('');

    const handleEmailInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        setEmailInput(event.target.value);
    };

    const [passwordInput, setPasswordInput] = useState('');
    const [finalPassword, setFinalPassword] = useState('');
    const handlePasswordInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        setPasswordInput(event.target.value);
    };

    const handleRegisterButton = () => {
        setFinalEmail(emailInput);
        setEmailInput('');
        setFinalPassword(passwordInput);
        setPasswordInput('');
    };


    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <MessageDisplay message={message}></MessageDisplay>
                <button onClick={handleCounterClick}>Change Message</button>

                <hr style={{margin: '20px 0'}}/>

                <h2>User Input Example</h2>
                <input type="email"
                       value={emailInput}
                       onChange={handleEmailInput}
                       placeholder="Enter email here"/>
                <input type="password"
                       value={passwordInput}
                       onChange={handlePasswordInput}
                       placeholder="Enter password here"/>
                <button onClick={handleRegisterButton}>Register</button>
                {finalEmail && finalPassword && (
                    <p>You submitted Email: <strong>{finalEmail}</strong>, Password = <strong>{finalPassword}</strong></p>
                )}
            </header>
        </div>
    )
}

export default App;
