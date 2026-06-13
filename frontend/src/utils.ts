export function validateEmailAndPassword(email: string, password: string, setError): boolean {
    setError('');
    const regex = /\S+@\S+\.\S+/;
    if (password.length < 6 || password.length > 255) {
        setError('Password length must be between 6 and 255 characters');
        return false;
    }

    else if (password.includes(' ')) {
        setError('Password may not contain any spaces');
        return false;
    }

    else if (!regex.test(email)) {
        setError('Invalid email');
        return false;
    }

    return true;
}
