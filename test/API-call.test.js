// Run with: node testApiCallback.js

const BASE_URL = "http://localhost:8080";

async function main() {
    const payload = {
        date: "2025-10-01",
        time: "20:30:00",
        batch: "LTC-123",
    };

    try {
        console.log("➡️ Sending POST to /encode...");
        const postRes = await fetch(`${BASE_URL}/encode`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        });

        if (!postRes.ok) {
            throw new Error(`POST failed: ${postRes.status} ${postRes.statusText}`);
        }

        const postData = await postRes.json();
        console.log("✅ POST Response:", postData);

        // Assuming the API returns something like { id: "abc123", ... }
        const id = postData.id;
        if (!id) throw new Error("❌ No ID returned from POST.");

        console.log(`\n➡️ Checking if record exists with ID: ${id}`);
        const getRes = await fetch(`${BASE_URL}/decode/${id}`);

        if (getRes.status === 404) {
            console.log("❌ Record not found.");
        } else if (!getRes.ok) {
            throw new Error(`GET failed: ${getRes.status} ${getRes.statusText}`);
        } else {
            const getData = await getRes.json();
            console.log("✅ GET Response:", getData);
        }
    } catch (err) {
        console.error("💥 Error:", err.message);
    }
}

main();
