interface Ok {
    kind: "ok",
    balance: string,
}

interface Err {
    kind: "err",
    message: string,
}

type BalanceResult = Ok | Err;

export async function balance(token: string): Promise<BalanceResult> {
    if (!token) {
        return {
            kind: "err",
            message: "no token",
        };
    }

    const request = {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${token}`,
        },
    };

    try {
        const response = await fetch("/balance", request);

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
        const message = err instanceof Error ? err.message : String(err);
        return {
            kind: "err",
            message: `Network Error: ${message}`
        };
    }
}
