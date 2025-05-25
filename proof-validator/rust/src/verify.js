const snarkjs = require("snarkjs");
const fs = require("fs");

async function verifyProof() {
    try {
        // Read the proof and verification key
        const proof = JSON.parse(fs.readFileSync("./proof.json", "utf8"));
        const verificationKey = JSON.parse(fs.readFileSync("./verification_key.json", "utf8"));

        // Transform the proof into the format expected by snarkjs
        const transformedProof = {
            pi_a: proof.proof.a,
            pi_b: proof.proof.b,
            pi_c: proof.proof.c
        };

        // Transform the verification key into the format expected by snarkjs
        const transformedVk = {
            IC: verificationKey.gamma_abc,
            vk_gamma_2: verificationKey.gamma,
            vk_delta_2: verificationKey.delta,
            vk_alpha_1: verificationKey.alpha,
            vk_beta_2: verificationKey.beta,
            curve: verificationKey.curve
        };

        // Verify the proof with a timeout
        const verificationPromise = snarkjs.groth16.verify(transformedVk, proof.inputs, transformedProof);
        const timeoutPromise = new Promise((_, reject) => {
            setTimeout(() => reject(new Error("Verification timed out after 30 seconds")), 30000);
        });

        const result = await Promise.race([verificationPromise, timeoutPromise]);

        if (result === true) {
            console.log("✅ Proof is valid!");
            process.exit(0);
        } else {
            console.log("❌ Proof is invalid!");
            process.exit(1);
        }
    } catch (error) {
        console.error("Error during verification:", error.message);
        console.error("Full error:", error);
        process.exit(1);
    }
}

verifyProof().catch((error) => {
    console.error("Unhandled error:", error);
    process.exit(1);
});
