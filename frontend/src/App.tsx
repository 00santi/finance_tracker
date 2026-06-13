import React, {useState} from 'react';
import { register } from './register.ts';
import { login } from './login.ts';
import { validateEmailAndPassword } from "./utils.ts";

function App() {
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
        if (!validateEmailAndPassword(emailInput, passwordInput, setErrorText))
            return;

        const result = await register(emailInput, passwordInput);
        setResultText(result.message);

        setFinalEmail(emailInput);
        setEmailInput('');
        setFinalPassword(passwordInput);
        setPasswordInput('');
    };

    const handleLoginButton = async () => {
        if (!validateEmailAndPassword(emailInput, passwordInput, setErrorText))
            return;

        const result = await login(emailInput, passwordInput);
        setResultText(result.message);
        if (result.kind === "ok") {
            localStorage.setItem("token", result.token);
            console.log("Saved token:", localStorage.getItem("token"));
        }
        setEmailInput('');
        setPasswordInput('');
    };

    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <hr style={{margin: '20px 0'}}/>
                <input type="email"
                       value={emailInput}
                       onChange={handleEmailInput}
                       placeholder="Enter email here"/><br/>
                <input type="password"
                       value={passwordInput}
                       onChange={handlePasswordInput}
                       placeholder="Enter password here"/><br/>
                <button onClick={handleRegisterButton}>Register</button>
                <button onClick={handleLoginButton}>Login</button>
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

export default App;
