# Multi-Token Invoice

This document covers the `ahjoor-payments` multi-token invoice module implemented
in `contracts/ahjoor-payments/src/multi_token_invoice.rs` and
`contracts/ahjoor-payments/src/multi_token_invoice_impl.rs`.

The module lets a merchant issue an invoice in one base currency, accept
payments in multiple approved tokens, optionally price those payments through
an oracle, and batch-settle fully paid invoices later.

## Core Model

An invoice stores:

- `merchant` and `customer`
- `total_amount`
- `base_currency`
- `accepted_tokens`
- `preferred_settlement_token`
- `line_items`
- `status`
- `payments_received`
- `conversion_rates`
- `settlement_conversion_rate`
- optional `oracle_contract`

The invoice status progresses through:

```text
Draft -> Issued -> PartiallyPaid -> FullyPaid
                         -> Cancelled
```

`create_invoice(...)` creates an invoice with `status = Issued`. The merchant
must sign the call.

## Invoice Creation

Use `create_invoice(...)` to open a new invoice:

```text
create_invoice(
    merchant,
    customer,
    total_amount,
    base_currency,
    accepted_tokens,
    preferred_settlement_token,
    line_items,
    due_date,
    metadata
)
```

Creation rules enforced by the contract:

- `merchant` must authorize the call.
- `total_amount` must be greater than zero.
- No more than 20 line items are allowed.
- Every line item must have positive `quantity` and `unit_price`.
- The invoice is stored with a unique `invoice_id`.

Each invoice line item captures:

- `description`
- `quantity`
- `unit_price`
- `amount`
- `tax_rate_bps`

## Per-Merchant Conversion Rates

Merchants configure token-to-base conversion rates with:

```text
set_conversion_rate(merchant, token, rate_to_base)
```

These rates are stored per merchant, not globally. A merchant can maintain a
different rate set from other merchants, and each accepted token can have its
own conversion rate.

Important behavior:

- `merchant` must authorize the call.
- `rate_to_base` must be greater than zero.
- The rate is stored in merchant-scoped invoice rate storage.
- `accept_payment(...)` requires a conversion rate for the chosen token.

The standard payment path uses this stored rate to convert the paid token amount
into invoice base currency.

## Standard Payment Flow

Use `accept_payment(...)` when the payer is paying with one of the accepted
invoice tokens and the merchant has already configured a conversion rate.

```text
accept_payment(invoice_id, payer, token, amount)
```

Contract checks:

- `payer` must authorize the call.
- `amount` must be greater than zero.
- The invoice must exist.
- The invoice cannot be `Cancelled` or already `FullyPaid`.
- The payment token must be in `accepted_tokens`.
- A conversion rate for the token must exist.

Conversion behavior:

- `amount_in_base = amount * conversion_rate / 1_000_000`
- `amount_in_settlement = amount_in_base * settlement_conversion_rate / 1_000_000`

The converted base amount is credited against the invoice balance. The contract
stores an `InvoicePayment` record and updates the invoice status:

- `PartiallyPaid` if balance remains
- `FullyPaid` if the total has been covered

## Oracle Price Feeds

Cross-token payment support is available through
`pay_invoice_cross_token(...)` when an oracle contract is configured on the
invoice.

```text
pay_invoice_cross_token(
    invoice_id,
    payer,
    payment_token,
    payment_amount,
    max_slippage_bps
)
```

Oracle behavior:

- The invoice must have `oracle_contract` set.
- The oracle is queried for the price of `payment_token` in terms of the
  invoice `base_currency`.
- The oracle price is expected to be scaled by `1_000_000`.
- If the oracle returns no price, the call fails.

The oracle-derived price is used to compute the amount credited in base
currency.

## Slippage Tolerance

The cross-token path compares the live oracle price against the merchant’s
stored conversion rate for that token, if one exists.

If the deviation exceeds `max_slippage_bps`, the call fails with
`SlippageExceeded`.

In practice, this means:

- The merchant can publish an expected rate through `set_conversion_rate(...)`.
- The oracle price is checked against that expected rate.
- The payer can cap tolerated drift with `max_slippage_bps`.

This prevents a payer from settling at a price that has moved too far away from
the merchant’s configured reference rate.

## Cross-Token Settlement Flow

The cross-token path performs the following steps:

1. Verify the invoice exists and is still payable.
2. Confirm the payment token is one of the invoice’s accepted tokens.
3. Require an oracle contract on the invoice.
4. Fetch the oracle price for the payment token against the base currency.
5. Enforce the slippage check against the stored merchant rate, when present.
6. Transfer `payment_amount` from the payer to the contract.
7. Convert the payment into base currency and update the invoice balance.
8. Store the payment and a `CrossTokenSettlementRecord`.
9. Emit the `CrossTokenSettlement` event.

The stored cross-token settlement record captures:

- `invoice_id`
- `paid_token`
- `paid_amount`
- `invoiced_token`
- `invoiced_amount`
- `oracle_price`
- `max_slippage_bps`

## Settlement to Merchant

After an invoice reaches `FullyPaid`, the merchant can settle invoices in batch
with:

```text
settle_invoices(merchant, invoice_ids)
```

Rules:

- `merchant` must authorize the call.
- A batch can include at most 50 invoices.
- Every invoice in the batch must belong to the merchant.
- Every invoice in the batch must already be `FullyPaid`.

The contract creates a `SettlementBatch` that records:

- `batch_id`
- `invoice_ids`
- `total_settlement_amount`
- `settled_at`
- `status`
- `merchant`

## Invoice Status and Balance

Helpful read APIs include:

- `get_invoice(invoice_id)`
- `get_invoice_status(invoice_id)`
- `get_invoice_balance(invoice_id)`
- `get_settlement_batch(batch_id)`

`get_invoice_balance(...)` returns the remaining unpaid base-currency amount for
the invoice.

## Cancellation

The merchant can cancel an invoice with:

```text
cancel_invoice(invoice_id)
```

After cancellation, the invoice can no longer be paid through either payment
path.

## Practical Notes

- Use `set_conversion_rate(...)` before expecting standard invoice payments.
- Use `set_settlement_conversion_rate(...)` when you want the settlement-side
  conversion factor reflected in payment accounting.
- Use `create_invoice_with_oracle(...)` if you want oracle-backed
  cross-token settlement from the start.
- Keep `accepted_tokens` narrow so invoice payments stay predictable.

