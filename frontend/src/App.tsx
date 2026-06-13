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

    const [errorText, setErrorText] = useState('');
    const [resultText, setResultText] = useState('');

    const handleRegisterButton = async () => {
        setErrorText('');

        const regex = /\S+@\S+\.\S+/;
        if (passwordInput.length < 6 || passwordInput.length > 255)
            setErrorText('Password length must be between 6 and 255 characters');
        else if (passwordInput.includes(' '))
            setErrorText('Password may not contain any spaces');
        else if (!regex.test(emailInput))
            setErrorText('Invalid email');
        else {
            const result = await register(emailInput, passwordInput);
            setResultText(result);

            setFinalEmail(emailInput);
            setEmailInput('');
            setFinalPassword(passwordInput);
            setPasswordInput('');
        }
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
                {errorText && (
                    <p>{errorText}</p>
                )}
                {resultText && (
                    <p>{resultText}</p>
                )}
            </header>
        </div>
    )
}

async function register(email: string, password: string): Promise<string> {
    try {
        const response = await fetch("http://localhost:7878/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                email: email,
                password: password,
            }),
        });

        const data = await response.json();

        if (!response.ok)
            return `Registration failed: ${data.message ?? 'Unknown error'}`;

        return (`Success! ${data.message}`);
    } catch (err) {
        return `Network Error: ${JSON.stringify(err)}`;
    }
}

export default App;
