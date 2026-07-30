# Bounty Board Feature Lifecycle

This document describes the bounty board lifecycle implemented in the
`ahjoor-escrow` contract. The feature allows a buyer to post a bounty for a
task, have solvers claim it, submit work, and receive payment upon approval.

## Status State Machine

```text
BountyUnclaimed ──> BountyClaimed ──> Released
     │  ^               │
     │  │               │ (reject)
     │  └───────────────┘
     │
     └──> Refunded
```

- **BountyUnclaimed** — bounty created, waiting for a solver to claim.
- **BountyClaimed** — a solver has claimed the bounty; work must be submitted
  before the submission deadline.
- **Released** — buyer approved the submission; funds transferred to solver.
- **Refunded** — bounty cancelled and funds returned to buyer.

## How a Bounty Is Posted and Funded

### Standard Bounty

Call:

```text
create_bounty(buyer, token, amount, description_hash, claim_deadline, submission_deadline)
```

The buyer signs the call and the contract transfers `amount` of `token` from the
buyer into the contract. The escrow is stored with status
`EscrowStatus::BountyUnclaimed`.

Validation rules:

- `amount` must be positive.
- `claim_deadline` must be in the future (greater than the current ledger
  timestamp).
- `submission_deadline` must be after `claim_deadline`.

A `BountyData` entry is created containing the `description_hash`,
`claim_deadline`, `submission_deadline`, and a zeroed `rejection_count`. No
solver is assigned yet.

### Milestone-Gated Bounty

For larger or multi-phase tasks, the buyer can split the bounty into milestones:

```text
create_milestone_bounty(buyer, token, milestones, claim_deadline, submission_deadline)
```

Each milestone specifies:

| Field | Meaning |
|---|---|
| `description_hash` | Hash of the sub-deliverable description. |
| `verifier` | Address authorized to sign off on this milestone. |
| `amount` | Token amount released to the solver when verified. |

The total of all milestone amounts is transferred from the buyer up front. All
milestones are stored with status `BountyMilestoneStatus::Pending`. The bounty
itself remains `BountyUnclaimed` until a solver claims it.

## How Submissions Are Made and Reviewed

### 1. Claiming the Bounty

Any solver can claim an unclaimed bounty on a first-come-first-served basis:

```text
claim_bounty(solver, escrow_id)
```

- Only valid while status is `BountyUnclaimed` and the current ledger timestamp
  is before `claim_deadline`.
- The escrow's `seller` is set to the solver and status moves to
  `EscrowStatus::BountyClaimed`.
- `BountyData.solver` is updated to `Some(solver)`.
- Once claimed, no other solver can claim the same bounty.

If the claim deadline passes without anyone claiming, the bounty remains
`BountyUnclaimed` and the buyer may cancel it for a refund.

### 2. Submitting Work

The assigned solver submits work by providing a hash of the deliverable:

```text
submit_bounty_work(solver, escrow_id, submission_hash)
```

- Only the assigned solver may call this.
- Only valid while status is `BountyClaimed` and the current ledger timestamp
  is before `submission_deadline`.
- The `submission_hash` is stored on `BountyData.submission_hash`.

#### Milestone Submission

For milestone bounties, the solver submits deliverables milestone by milestone
in order:

```text
submit_bounty_milestone(solver, escrow_id, index, deliverable_hash)
```

- Only the assigned solver may call this.
- Milestones must be submitted sequentially (all earlier milestones must be
  `Paid` before the next one can be submitted).
- The milestone status moves from `Pending` to `Submitted`.

#### Milestone Verification

Each milestone has a designated verifier who signs off:

```text
verify_bounty_milestone(escrow_id, index)
```

- Only the milestone's designated `verifier` (authenticated via `require_auth`)
  may call this.
- Only valid while the milestone status is `Submitted`.
- On success, the milestone's `amount` is transferred to the solver, the
  milestone is marked `Paid`, and `fees_disbursed` is incremented.
- When all milestones are paid, the escrow status moves to `Released`.

The buyer may replace a verifier before their milestone is submitted:

```text
replace_bounty_verifier(buyer, escrow_id, index, new_verifier)
```

### 3. Approving or Rejecting the Submission

Once work has been submitted, the buyer reviews it.

#### Approval

```text
approve_bounty_submission(buyer, escrow_id)
```

- Only the buyer may call this.
- Requires that `BountyData.submission_hash` is `Some` (work has been
  submitted).
- The full escrow amount is transferred to the solver, the escrow status moves
  to `EscrowStatus::Released`, and `fees_disbursed` is incremented.

#### Rejection

```text
reject_bounty_submission(buyer, escrow_id)
```

- Only the buyer may call this.
- Requires that a submission has been made.
- On rejection:
  - The solver is cleared (`BountyData.solver` set to `None`).
  - The submission hash is cleared.
  - `rejection_count` is incremented.
  - Status resets to `BountyUnclaimed`, allowing a new solver to claim.

If `rejection_count` reaches the maximum (default `3`, configurable by admin),
the bounty enters a terminal state — further rejection is blocked and the
bounty remains stuck (the buyer must cancel for a refund at this point).

#### Configuring Max Rejection Rounds

The admin may adjust the maximum rejection count:

```text
set_max_bounty_rejection_rounds(admin, max_rounds)
```

## How the Bounty Is Paid Out or Cancelled/Refunded

### Normal Pay Out

The happy path ends with `approve_bounty_submission` transferring funds to the
solver and status reaching `Released`.

For milestone bounties, the solver is paid incrementally as each milestone is
verified, culminating in `Released` when all milestones are complete.

### Cancellation

The buyer may cancel a bounty and receive a refund:

```text
cancel_bounty(buyer, escrow_id)
```

Cancellation is allowed only in specific states:

| Status | Cancellable? | Notes |
|---|---|---|
| `BountyUnclaimed` | Yes | Full refund of the bounty amount. |
| `BountyClaimed` (past claim deadline) | Yes | Refunds `amount - fees_disbursed`. |
| `BountyClaimed` (before claim deadline ends) | No | Solver has an active claim. |
| `Released` | No | Funds already disbursed. |

On cancellation, the remaining escrowed balance (`amount - fees_disbursed`) is
transferred back to the buyer and the escrow status moves to `Refunded`.

## Events

| Event | When Emitted |
|---|---|
| `BountyCreated` | A new bounty is posted. |
| `BountyClaimed` | A solver claims the bounty. |
| `BountyWorkSubmitted` | The solver submits work. |
| `BountySubmissionApproved` | The buyer approves the submission. |
| `BountySubmissionRejected` | The buyer rejects the submission. |
| `BountyCancelled` | The buyer cancels the bounty. |
| `BountyMilestoneSubmitted` | A milestone deliverable is submitted. |
| `BountyMilestoneVerified` | A milestone is verified and paid. |

## BountyData Structure

| Field | Type | Meaning |
|---|---|---|
| `description_hash` | `BytesN<32>` | Hash of the bounty task description. |
| `claim_deadline_ledger` | `u64` | Ledger timestamp deadline for claiming. |
| `submission_deadline_ledger` | `u64` | Ledger timestamp deadline for submitting work. |
| `solver` | `Option<Address>` | Address of the solver who claimed the bounty. |
| `submission_hash` | `Option<BytesN<32>>` | Hash of the submitted work. |
| `rejection_count` | `u32` | Number of times this bounty has been rejected. |
| `fees_disbursed` | `i128` | Bounty funds already paid out (for milestone bounties). |

## Integration Notes

- Before creating a bounty, the buyer must approve the escrow contract to
  transfer the bounty token amount.
- The `description_hash` should reference off-chain content (e.g., IPFS or
  similar) that contains the full bounty specification.
- Both `claim_deadline` and `submission_deadline` use ledger timestamps — render
  these as countdowns in any UI, not wall-clock times.
- The default maximum rejection rounds is `3`. After exhausting all rounds,
  the buyer cannot reject again and must cancel the bounty for a refund.
- For milestone bounties, the verifier for each milestone is set at creation
  time and cannot be changed after the milestone has been submitted.
