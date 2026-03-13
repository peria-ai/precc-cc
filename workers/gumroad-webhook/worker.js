/**
 * PRECC Gumroad Webhook — Cloudflare Worker
 *
 * Receives Gumroad purchase pings and returns a per-user license key.
 *
 * Setup:
 *   wrangler secret put LICENSE_SECRET    # same as PRECC_LICENSE_SECRET
 *   wrangler secret put GUMROAD_SELLER_ID # your Gumroad seller ID
 *   wrangler deploy
 *
 * Gumroad ping docs: https://gumroad.com/ping
 */

export default {
  async fetch(request, env) {
    if (request.method !== "POST") {
      return new Response("Method not allowed", { status: 405 });
    }

    const url = new URL(request.url);
    if (url.pathname !== "/webhook") {
      return new Response("Not found", { status: 404 });
    }

    try {
      const formData = await request.formData();
      const email = formData.get("email");
      const sellerId = formData.get("seller_id");

      // Verify the request is from our Gumroad account
      if (!sellerId || sellerId !== env.GUMROAD_SELLER_ID) {
        return new Response("Unauthorized", { status: 403 });
      }

      if (!email) {
        return new Response("Missing email", { status: 400 });
      }

      // Generate license key
      const normalizedEmail = email.trim().toLowerCase();
      const editionFlags = 1; // Pro
      const expiryDays = 0;   // No expiry

      const key = await generateKey(env.LICENSE_SECRET, normalizedEmail, expiryDays, editionFlags);

      return new Response(
        JSON.stringify({
          success: true,
          license_key: key,
          email: normalizedEmail,
          edition: "Pro",
          activate: `precc license activate ${key} --email ${normalizedEmail}`,
        }),
        {
          status: 200,
          headers: { "Content-Type": "application/json" },
        }
      );
    } catch (err) {
      return new Response(JSON.stringify({ error: err.message }), {
        status: 500,
        headers: { "Content-Type": "application/json" },
      });
    }
  },
};

/**
 * Generate a PRECC license key matching the Rust implementation.
 *
 * Key = PRECC-{tag(4)}{expiry(4)}{flags(4)}{mac(4)} as uppercase hex with dashes.
 */
async function generateKey(secret, email, expiryDays, editionFlags) {
  const encoder = new TextEncoder();

  // Compute email fingerprint: SHA-256(email)[0..4]
  const emailHash = await crypto.subtle.digest("SHA-256", encoder.encode(email));
  const emailTag = new Uint8Array(emailHash).slice(0, 4);

  // Build 12-byte payload: [tag(4) | expiry(4) | flags(4)]
  const payload = new Uint8Array(12);
  payload.set(emailTag, 0);
  // expiry_days as big-endian u32
  payload[4] = (expiryDays >>> 24) & 0xff;
  payload[5] = (expiryDays >>> 16) & 0xff;
  payload[6] = (expiryDays >>> 8) & 0xff;
  payload[7] = expiryDays & 0xff;
  // edition_flags as big-endian u32
  payload[8] = (editionFlags >>> 24) & 0xff;
  payload[9] = (editionFlags >>> 16) & 0xff;
  payload[10] = (editionFlags >>> 8) & 0xff;
  payload[11] = editionFlags & 0xff;

  // HMAC-SHA256(secret, payload)[0..4]
  const hmacKey = await crypto.subtle.importKey(
    "raw",
    encoder.encode(secret),
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"]
  );
  const signature = await crypto.subtle.sign("HMAC", hmacKey, payload);
  const mac = new Uint8Array(signature).slice(0, 4);

  // Concatenate into 16 bytes
  const keyBytes = new Uint8Array(16);
  keyBytes.set(payload, 0);
  keyBytes.set(mac, 12);

  // Format as PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX
  const hex = Array.from(keyBytes)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("")
    .toUpperCase();

  return `PRECC-${hex.slice(0, 8)}-${hex.slice(8, 16)}-${hex.slice(16, 24)}-${hex.slice(24, 32)}`;
}
