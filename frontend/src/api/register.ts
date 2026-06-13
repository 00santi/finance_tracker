interface Ok {
    kind: "ok",
    message: string,
}

interface Err {
    kind: "err",
    message: string,
}

type RegisterResult = Ok | Err;

export async function register(email: string, password: string): Promise<RegisterResult> {
    try {
        const response = await fetch("http://localhost:7878/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username: null,
                email: email,
                password: password,
            }),
        });

        if (!response.ok) {
            const errText = await response.text();
            return {
                kind: "err",
                message: `Registration failed: ${errText || 'Unknown error'}`
            };
        }

        return {
            kind: "ok",
            message: `Registered successfully`
        };
    } catch (err) {
        return {
            kind: "err",
            message: `Network Error: ${err.message}`
        };
    }
}
