import React, {useState} from 'react';
import { register } from './api/register.ts';
import { login } from './api/login.ts';
import { validateEmailAndPassword } from "./utils.ts";
import Dashboard from "./Dashboard.tsx"

function App() {
    const [token, setToken] = useState<string | null>(() => { return localStorage.getItem("token"); });

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
        } else {
            setResultText(result.message);
        }
        setEmail('');
        setPassword('');
    };

    const handleLogoutButton = async () => {
        setToken(null);
        setErrorText("");
        setResultText("");
        localStorage.removeItem("token");
    };

    if (token) {
        return <Dashboard username={email}
                          logoutHandler={handleLogoutButton} />;
    } else {
        return loginRegisterPage(email, handleEmailInput, password, handlePasswordInput, handleRegisterButton, handleLoginButton, errorText,  resultText);
    }
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
