import React, {useState} from 'react';
import { register } from './api/register.ts';
import { login } from './api/login.ts';
import { validateEmailAndPassword } from "./utils.ts";

function App() {
    const [token, setToken] = useState<string | null>(null);

    const [email, setEmail] = useState('');
    const handleEmailInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        setEmail(event.target.value);
    };

    const [password, setPassword] = useState('');
    const handlePasswordInput = (event: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(event.target.value);
    };

    const [errorText, setErrorText] = useState('');
    const [resultText, setResultText] = useState('');

    const handleRegisterButton = async () => {
        if (!validateEmailAndPassword(email, password, setErrorText))
            return;

        const result = await register(email, password);
        setResultText(result.message);

        setEmail('');
        setPassword('');
    };

    const handleLoginButton = async () => {
        if (!validateEmailAndPassword(email, password, setErrorText))
            return;

        const result = await login(email, password);
        if (result.kind === "ok") {
            setResultText("Login successful!");
            setToken(result.token);
            localStorage.setItem("token", result.token);
            console.log("Saved token:", localStorage.getItem("token"));
        } else {
            setResultText(result.message);
        }
        setEmail('');
        setPassword('');
    };

    if (token) {
        return dashboardPage;
    } else {
        return loginRegisterPage(email, handleEmailInput, password, handlePasswordInput, handleRegisterButton, handleLoginButton, errorText,  resultText);
    }
}

function dashboardPage() {
    return (
        <div className="Dashboard">
            <h1>Dashboard Page</h1>
        </div>
    )
}

function loginRegisterPage(email, emailHandler, password, passwordHandler, registerHandler, loginHandler, errorText, resultText) {
    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <hr style={{margin: '20px 0'}}/>
                <input type="email"
                       value={email}
                       onChange={emailHandler}
                       placeholder="Enter email here"/><br/>
                <input type="password"
                       value={password}
                       onChange={passwordHandler}
                       placeholder="Enter password here"/><br/>
                <button onClick={registerHandler}>Register</button>
                <button onClick={loginHandler}>Login</button>
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
