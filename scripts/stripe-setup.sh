#!/bin/bash
# stripe-setup.sh — Create PRECC Pro products and payment links on Stripe.
#
# Prerequisites:
#   - Stripe CLI installed (brew install stripe/stripe-cli/stripe)
#   - Logged in (stripe login)
#
# Creates:
#   1. PRECC Pro (30-day) — $1
#   2. PRECC Pro (12-month) — $10
#
# Usage:
#   bash scripts/stripe-setup.sh              # test mode (default)
#   bash scripts/stripe-setup.sh --live       # live mode

set -euo pipefail

MODE="test"
if [[ "${1:-}" == "--live" ]]; then
    MODE="live"
    echo "==> LIVE MODE — charges will be real"
else
    echo "==> TEST MODE — no real charges"
fi

SUCCESS_URL="https://github.com/yijunyu/precc-cc?session_id={CHECKOUT_SESSION_ID}"

# ---------------------------------------------------------------------------
# Product
# ---------------------------------------------------------------------------
echo "==> Creating PRECC Pro product..."
PRODUCT_JSON=$(stripe products create \
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
# Price 1: 30-day — $1
# ---------------------------------------------------------------------------
echo "==> Creating 30-day price (\$1)..."
PRICE_30D_JSON=$(stripe prices create \
    --product "${PRODUCT_ID}" \
    --unit-amount 100 \
    --currency usd)
PRICE_30D=$(echo "${PRICE_30D_JSON}" | jq -r '.id')
if [[ -z "${PRICE_30D}" || "${PRICE_30D}" == "null" ]]; then
    echo "ERROR: Failed to create 30-day price:" >&2
    echo "${PRICE_30D_JSON}" >&2
    exit 1
fi
echo "  Price (30-day): ${PRICE_30D}"

# ---------------------------------------------------------------------------
# Price 2: 12-month — $10
# ---------------------------------------------------------------------------
echo "==> Creating 12-month price (\$10)..."
PRICE_12M_JSON=$(stripe prices create \
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

LINK_30D_JSON=$(stripe payment_links create \
    -d "line_items[0][price]=${PRICE_30D}" \
    -d "line_items[0][quantity]=1" \
    -d "metadata[edition]=pro" \
    -d "metadata[expiry_days]=30" \
    --after-completion.type=redirect \
    "--after-completion.redirect.url=${SUCCESS_URL}")
LINK_30D=$(echo "${LINK_30D_JSON}" | jq -r '.url')
if [[ -z "${LINK_30D}" || "${LINK_30D}" == "null" ]]; then
    echo "ERROR: Failed to create 30-day payment link:" >&2
    echo "${LINK_30D_JSON}" >&2
    exit 1
fi

LINK_12M_JSON=$(stripe payment_links create \
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
echo "  30-day (\$1):"
echo "    Price:     ${PRICE_30D}"
echo "    Link:      ${LINK_30D}"
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
echo "       (append it or use a separate file)"
echo "    3. Ensure metadata.expiry_days is passed through Checkout Sessions"
echo ""
echo "  Test a purchase:"
echo "    - Open a payment link in your browser"
echo "    - Use card 4242 4242 4242 4242, any future expiry, any CVC"
echo "    - Copy the cs_test_XXXXX from the redirect URL"
echo "    - Run: precc license activate cs_test_XXXXX"
echo ""
