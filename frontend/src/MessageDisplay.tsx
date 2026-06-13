import React from 'react';

interface MessageDisplayProps {
    message: string;
}

const MessageDisplay: React.FC<MessageDisplayProps> = ( { message } ) => {
    return (
        <p>{message}</p>
    );
};

export default MessageDisplay;