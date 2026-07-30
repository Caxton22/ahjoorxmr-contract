# Seller Veto Mechanism

The **Seller Veto** allows a seller in an active escrow dispute to temporarily halt auto-resolution or immediate buyer refund triggers, giving both parties time to negotiate or submit evidence for platform arbitration.

## 1. Scope & Blocked Actions
When a seller exercises a veto on a disputed transaction, the contract temporarily blocks:
* **Automated Buyer Refund Execution**
* **Immediate Dispute Closure**
* **Fund Release to Buyer**

## 2. Duration & Cooldown Rules
| Parameter | Type | Default / Limit |
| :--- | :--- | :--- |
| `veto_duration` | `u64` (seconds / ledgers) | Configurable (e.g., 7 days) |
| `veto_cooldown` | `u64` (seconds / ledgers) | Configurable (e.g., 14 days) |
| `max_vetoes_per_dispute` | `u32` | **1 veto per dispute** |

## 3. Resolution Pathways
1. **Veto Expiration:** Times out automatically after `veto_duration`.
2. **Arbitrator Override:** Admin calls `force_resolve_dispute`.
3. **Seller Cancellation:** Seller invokes `cancel_veto`.
