/*
    POST
*/

interface PostOk {
    kind: "ok",
}

interface PostErr {
    kind: "err",
    message: string,
}

type PostResult = PostOk | PostErr;

export async function postTransactions(token: string, amount: number, category: string, description: string | null): Promise<PostResult> {
    if (!token) {
        return {
            kind: "err",
            message: "no token",
        };
    }

    const request = {
        method: "POST",
        headers: {
            "Authorization": `Bearer ${token}`,
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            amount: amount,
            category: category,
            description: description,
        })
    };

    try {
        const response = await fetch("/transactions", request);

        if (!response.ok) {
            const errText = await response.text();
            return {
                kind: "err",
                message: `Error saving transaction: ${errText}`
            };
        }
        
        return { kind: "ok" };
    }
    catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        return {
            kind: "err",
            message: `Network Error: ${message}`
        };
    }
}

/*
    GET
*/

export interface Transaction {
    amount: string,
    category: string,
    description: string | null,
    created_at: string,
}

interface GetOk {
    kind: "ok",
    transactions: Transaction[],
}

interface GetErr {
    kind: "err",
    message: string,
}

type GetResult = GetOk | GetErr;

export async function getTransactions(token: string): Promise<GetResult> {
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
        const response = await fetch("/transactions", request);

        if (!response.ok) {
            const errText = await response.text();
            return {
                kind: "err",
                message: `Error fetching transactions: ${errText}`
            };
        }

        const data = await response.json();
        return { kind: "ok", transactions: data };
    }
    catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        return {
            kind: "err",
            message: `Network Error: ${message}`
        };
    }
}
