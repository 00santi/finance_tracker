import React from 'react';

interface LoginRegisterProps {
    email: string;
    setEmail: (email: string) => void;
    password: string;
    setPassword: (password: string) => void;
    registerHandler: () => void;
    loginHandler: () => void;
    errorText: string;
    resultText: string;
}

const LoginRegisterPage: React.FC<LoginRegisterProps> = ({email, setEmail, password, setPassword, registerHandler, loginHandler, errorText, resultText}) => {
    return (
        <div className="App">
            <header className="App-header">
                <h1>Finance Tracker Frontend</h1>
                <hr style={{margin: '20px 0'}}/>
                <input type="email"
                       value={email}
                       onChange={(e) => setEmail(e.target.value)}
                       placeholder="Enter email here"/><br/>
                <input type="password"
                       value={password}
                       onChange={(e) => setPassword(e.target.value)}
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
    );
};

export default LoginRegisterPage;
