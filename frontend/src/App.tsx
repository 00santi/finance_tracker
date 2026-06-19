import React, {useState} from 'react';
import { register } from './api/register.ts';
import { login } from './api/login.ts';
import { validateEmailAndPassword } from "./utils.ts";
import Dashboard from "./Dashboard.tsx"
import LoginRegisterPage from "./LoginRegisterPage.tsx"

function App() {
    const [token, setToken] = useState<string | null>(() => { return localStorage.getItem("token"); });

    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [errorText, setErrorText] = useState<string>('');
    const [resultText, setResultText] = useState<string>('');

    const handleRegisterButton = async () => {
        if (!validateEmailAndPassword(email, password, setErrorText)) {
            setEmail("");
            setPassword("");
            return;
        }

        const result = await register(email, password);
        setResultText(result.message);
    };

    const handleLoginButton = async () => {
        if (!validateEmailAndPassword(email, password, setErrorText)) {
            setEmail('');
            setPassword('');
            return;
        }

        const result = await login(email, password);
        if (result.kind === "ok") {
            setResultText("Login successful!");
            setToken(result.token);
            localStorage.setItem("token", result.token);
        } else {
            setResultText(result.message);
        }
    };

    const handleLogoutButton = async () => {
        setToken(null);
        setErrorText("");
        setResultText("");
        localStorage.removeItem("token");
    };

    if (token) {
        return <Dashboard username={email}
                          token={token}
                          logoutHandler={handleLogoutButton} />;
    }
    else {
        return <LoginRegisterPage email={email}
                                  setEmail={setEmail}
                                  password={password}
                                  setPassword={setPassword}
                                  registerHandler={handleRegisterButton}
                                  loginHandler={handleLoginButton}
                                  errorText={errorText}
                                  resultText={resultText} />;
    }
}

export default App;
