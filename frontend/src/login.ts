interface Ok {
    kind: "ok",
    message: string,
    token: string,
}

interface Err {
    kind: "err",
    message: string,
}

type LoginResult = Ok | Err;

export async function login(email: string, password: string): Promise<LoginResult> {
    try {
        const response = await fetch("http://localhost:7878/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                email: email,
                password: password,
            }),
        });

        if (!response.ok) {
            const errText = await response.text();
            return {
                kind: "err",
                message: `Login failed: ${errText || 'Unknown error'}`
            };
        }
        const data = await response.json();
        return {
            kind: "ok",
            message: `Success! Token: ${data.access_token}`,
            token: data.access_token,
        };
    } catch (err) {
        return {
            kind: "err",
            message: `Network Error: ${JSON.stringify(err)}`
        };
    }
}
