interface Ok {
    kind: "ok",
    balance: string,
}

interface Err {
    kind: "err",
    message: string,
}

type BalanceResult = Ok | Err;

export async function balance(token: string | null): Promise<BalanceResult> {
    if (!token)
        token = localStorage.getItem("token");
    if (!token) {
        return {
            kind: "err",
            message: "invalid token",
        };
    }

    const request = {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`,
        },
    };

    try {
        const response = await fetch("http://localhost:7878/balance", request);

        if (!response.ok) {
            const errText = await response.text();
            return {
                kind: "err",
                message: `Error fetching balance: ${errText}`
            };
        }

        const data = await response.json();
        return { kind: "ok", balance: data.balance };
    }
    catch (err) {
        return {
            kind: "err",
            message: `Network Error: ${err.message}`
        };
    }
}
