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
        const response = await fetch("/register", {
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
        const message = err instanceof Error ? err.message : String(err);
        return {
            kind: "err",
            message: `Network Error: ${message}`
        };
    }
}
