#!/bin/bash
# stripe-setup.sh — Create PRECC Pro products and payment links on Stripe.
#
# Prerequisites:
#   - Stripe CLI installed (brew install stripe/stripe-cli/stripe)
#   - Logged in (stripe login)
#
# Creates:
#   1. PRECC Pro (6-month) — $5
#   2. PRECC Pro (12-month) — $10
#
# Usage:
#   bash scripts/stripe-setup.sh              # test mode (default)
#   bash scripts/stripe-setup.sh --live       # live mode

set -euo pipefail

MODE="test"
STRIPE_FLAG=""
if [[ "${1:-}" == "--live" ]]; then
    MODE="live"
    STRIPE_FLAG="--live"
    echo "==> LIVE MODE — charges will be real"
else
    echo "==> TEST MODE — no real charges"
fi

SUCCESS_URL="https://github.com/peria-ai/precc-cc?session_id={CHECKOUT_SESSION_ID}"

# ---------------------------------------------------------------------------
# Product
# ---------------------------------------------------------------------------
echo "==> Creating PRECC Pro product..."
PRODUCT_JSON=$(stripe products create ${STRIPE_FLAG} \
    --name "PRECC Pro" \
    --description "Predictive Error Correction for Claude Code — Pro license with unlimited skills, detailed savings, email, and GIF generation.")
PRODUCT_ID=$(echo "${PRODUCT_JSON}" | jq -r '.id')
if [[ -z "${PRODUCT_ID}" || "${PRODUCT_ID}" == "null" ]]; then
    echo "ERROR: Failed to create product:" >&2
    echo "${PRODUCT_JSON}" >&2
    exit 1
fi
echo "  Product: ${PRODUCT_ID}"

# ---------------------------------------------------------------------------
# Price 1: 6-month — $5
# ---------------------------------------------------------------------------
echo "==> Creating 6-month price (\$5)..."
PRICE_6M_JSON=$(stripe prices create ${STRIPE_FLAG} \
    --product "${PRODUCT_ID}" \
    --unit-amount 500 \
    --currency usd)
PRICE_6M=$(echo "${PRICE_6M_JSON}" | jq -r '.id')
if [[ -z "${PRICE_6M}" || "${PRICE_6M}" == "null" ]]; then
    echo "ERROR: Failed to create 6-month price:" >&2
    echo "${PRICE_6M_JSON}" >&2
    exit 1
fi
echo "  Price (6-month): ${PRICE_6M}"

# ---------------------------------------------------------------------------
# Price 2: 12-month — $10
# ---------------------------------------------------------------------------
echo "==> Creating 12-month price (\$10)..."
PRICE_12M_JSON=$(stripe prices create ${STRIPE_FLAG} \
    --product "${PRODUCT_ID}" \
    --unit-amount 1000 \
    --currency usd)
PRICE_12M=$(echo "${PRICE_12M_JSON}" | jq -r '.id')
if [[ -z "${PRICE_12M}" || "${PRICE_12M}" == "null" ]]; then
    echo "ERROR: Failed to create 12-month price:" >&2
    echo "${PRICE_12M_JSON}" >&2
    exit 1
fi
echo "  Price (12-month): ${PRICE_12M}"

# ---------------------------------------------------------------------------
# Payment Links
# ---------------------------------------------------------------------------
echo ""
echo "==> Creating payment links..."

LINK_6M_JSON=$(stripe payment_links create ${STRIPE_FLAG} \
    -d "line_items[0][price]=${PRICE_6M}" \
    -d "line_items[0][quantity]=1" \
    -d "metadata[edition]=pro" \
    -d "metadata[expiry_days]=180" \
    --after-completion.type=redirect \
    "--after-completion.redirect.url=${SUCCESS_URL}")
LINK_6M=$(echo "${LINK_6M_JSON}" | jq -r '.url')
if [[ -z "${LINK_6M}" || "${LINK_6M}" == "null" ]]; then
    echo "ERROR: Failed to create 6-month payment link:" >&2
    echo "${LINK_6M_JSON}" >&2
    exit 1
fi

LINK_12M_JSON=$(stripe payment_links create ${STRIPE_FLAG} \
    -d "line_items[0][price]=${PRICE_12M}" \
    -d "line_items[0][quantity]=1" \
    -d "metadata[edition]=pro" \
    -d "metadata[expiry_days]=365" \
    --after-completion.type=redirect \
    "--after-completion.redirect.url=${SUCCESS_URL}")
LINK_12M=$(echo "${LINK_12M_JSON}" | jq -r '.url')
if [[ -z "${LINK_12M}" || "${LINK_12M}" == "null" ]]; then
    echo "ERROR: Failed to create 12-month payment link:" >&2
    echo "${LINK_12M_JSON}" >&2
    exit 1
fi

echo ""
echo "==========================================="
echo "  PRECC Pro Stripe Setup Complete"
echo "==========================================="
echo ""
echo "  Product:     ${PRODUCT_ID}"
echo ""
echo "  6-month (\$5):"
echo "    Price:     ${PRICE_6M}"
echo "    Link:      ${LINK_6M}"
echo ""
echo "  12-month (\$10):"
echo "    Price:     ${PRICE_12M}"
echo "    Link:      ${LINK_12M}"
echo ""
echo "  Success URL: ${SUCCESS_URL}"
echo ""
echo "  Next steps:"
echo "    1. Add payment links to your README / pricing page"
echo "    2. Set PRECC_STRIPE_SECRET_KEY in ~/.config/precc/build-secret"
echo "    3. Webhook auto-sends license keys to customers via support@peria.ai"
echo ""
