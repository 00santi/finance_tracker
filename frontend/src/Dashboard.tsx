import React, {useState} from 'react';
import {balance} from './api/balance.ts'
import {getTransactions, postTransactions, type Transaction} from './api/transactions.ts'

interface DashboardProps {
    username: string;
    token: string;
    logoutHandler: () => void;
}

const Dashboard: React.FC<DashboardProps> = ({username, token, logoutHandler}) => {
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<string | null>(null);

    const [currentBalance, setBalance] = useState<string>("");
    const fetchBalance = async () => {
        const result = await balance(token);
        if (result.kind == "err") {
            setSuccess(null);
            setError(result.message);
        } else {
            setError(null);
            setBalance(result.balance);
        }
    };

    const [history, setHistory] = useState<Transaction[] | null>(null);
    const fetchTransactions = async () => {
        const result = await getTransactions(token);
        if (result.kind === "err") {
            setSuccess(null);
            setError(result.message);
        } else {
            setError(null);
            setHistory(result.transactions);
            if (history === []) setSuccess("No transaction history");
        }
    };

    const [transaction, setTransaction] = useState<string>("");
    const [category, setCategory] = useState<string>("PERSONAL");
    const [description, setDescription] = useState<string>("");

    const addTransaction = async () => {
        if (!category || !transaction) {
            setError("Please enter a transaction amount and category");
            setSuccess(null);
            return;
        }
        const parsed = Number(transaction);
        if (isNaN(parsed)) {
            setError("Please enter a numeric amount");
            setSuccess(null);
            return;
        }
        const result = await postTransactions(token, parsed, category, description);
        if (result.kind === "err") {
            setSuccess(null);
            setError(result.message);
        } else {
            setError(null);
            setSuccess("Transaction added");
        }
        setTransaction("");
        setDescription("");
    };

    return (
        <div>
            <h2>Welcome to your Dashboard!</h2>
            <p>User: {username}</p>
            <p>Balance: {currentBalance}</p>
            {error && <p style={{color: 'red'}}>{error}</p>}
            <input type="number"
                   value={transaction}
                   onChange={(e) => setTransaction(e.target.value)}
                   placeholder="Enter transaction amount here"/><br/>
            {success && <p style={{color: 'green'}}>{success}</p>}
            <select value={category}
                    onChange={(e) => setCategory(e.target.value)}>
                <option value="PERSONAL">PERSONAL</option>
                <option value="BUSINESS">BUSINESS</option>
                <option value="PAYCHECK">PAYCHECK</option>
                <option value="TRAVEL">TRAVEL</option>
                <option value="RENT">RENT</option>
                <option value="GROCERIES">GROCERIES</option>
            </select>
            <input type="text"
                   value={description}
                   onChange={(e) => setDescription(e.target.value)}
                   placeholder="(Optional) Description transaction"/><br/>
            <button onClick={addTransaction}>Add New Transaction</button>
            <button onClick={fetchTransactions}>See Transaction History</button>
            <button onClick={fetchBalance}>Update Balance</button>
            <button onClick={logoutHandler}>Log out</button>
            {history && history.map((t) => (
                <div key={t.created_at}>
                    {t.description}: {t.amount} ({t.category})
                </div>
            ))}
        </div>
    );
};

export default Dashboard;
