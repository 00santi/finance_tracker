import React from 'react';

interface DashboardProps {
    authToken: string;
    logoutHandler: () => void;
}

const Dashboard: React.FC<DashboardProps> = ({authToken, logoutHandler}) => {
    return (
        <div>
            <h2>Welcome to your Dashboard!</h2>
            <p>Auth Token: {authToken.substring(0, 10)}...</p>
            <button>Add New Transaction</button>
            <button>Get Transaction History</button>
            <button>Get Balance</button>
            <button onClick={logoutHandler}>Log out</button>
        </div>
    );
};

export default Dashboard;