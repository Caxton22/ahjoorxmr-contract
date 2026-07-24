//! # ahjoor-errors — Global Error Code Namespace Registry
//!
//! Each Ahjoor protocol contract owns a non-overlapping numeric range so that
//! off-chain parsers can unambiguously decode `InvokeHostFunctionTrapped` errors
//! without per-contract decode tables.
//!
//! ## Range allocation
//!
//! | Contract              | Range       |
//! |----------------------|-------------|
//! | ahjoor-rosca         | 1000 – 1299 |
//! | ahjoor-payments      | 2000 – 2299 |
//! | ahjoor-escrow        | 3000 – 3299 |
//! | ahjoor-refund        | 4000 – 4099 |
//! | ahjoor-token-whitelist | 5000 – 5099 |
//!
//! On-chain contracts continue to use their existing small discriminants (1–118
//! for rosca, 1–56 for payments, etc.) because `#[contracterror]` must produce
//! values that fit in the Soroban XDR `ScError` u32 field and the existing enum
//! variants are already deployed.  This crate provides the *off-chain* namespace
//! that relay nodes and indexers use when decoding errors across contracts.

// ---------------------------------------------------------------------------
// ahjoor-rosca (1000–1299)
// ---------------------------------------------------------------------------

pub mod rosca {
    /// Number of `pub const` codes declared in this module. Kept in sync
    /// manually; `ahjoor-errors`'s test suite cross-checks this against the
    /// number of `ALL_ERRORS` entries tagged `"ahjoor-rosca"` so a code added
    /// here without a matching `ALL_ERRORS` entry (or vice versa) fails CI.
    pub const COUNT: usize = 110;

    // Core Error variants (on-chain discriminant → namespaced code)
    pub const ALREADY_INITIALIZED: u32         = 1001;
    pub const TOKEN_NOT_APPROVED: u32          = 1002;
    pub const CUSTOM_ORDER_LENGTH_MISMATCH: u32 = 1003;
    pub const CUSTOM_ORDER_NON_MEMBER: u32     = 1004;
    pub const AMOUNT_MUST_BE_POSITIVE: u32     = 1005;
    pub const ROUND_DEADLINE_PASSED: u32       = 1006;
    pub const MEMBER_HAS_EXITED: u32           = 1007;
    pub const NOT_A_MEMBER: u32                = 1008;
    pub const ALREADY_CONTRIBUTED: u32         = 1009;
    pub const INVALID_EXCHANGE_RATE: u32       = 1010;
    pub const EXCEEDS_TOKEN_LIMIT: u32         = 1011;
    pub const EXCEEDS_REMAINING_CONTRIBUTION: u32 = 1012;
    pub const DEADLINE_NOT_PASSED: u32         = 1013;
    pub const PENALTY_DISABLED: u32            = 1014;
    pub const NOT_A_DEFAULTER: u32             = 1015;
    pub const CANNOT_CHANGE_MID_ROUND: u32     = 1016;
    pub const ALREADY_A_MEMBER: u32            = 1017;
    pub const NO_REWARDS_TO_CLAIM: u32         = 1018;
    pub const ONLY_MEMBERS_ALLOWED: u32        = 1019;
    pub const PROPOSAL_NOT_FOUND: u32          = 1020;
    pub const VOTING_DEADLINE_PASSED: u32      = 1021;
    pub const PROPOSAL_NOT_PENDING: u32        = 1022;
    pub const ALREADY_VOTED: u32               = 1023;
    pub const VOTING_NOT_ENDED: u32            = 1024;
    pub const CONTRACT_PAUSED: u32             = 1025;
    pub const ALL_MEMBERS_SUSPENDED: u32       = 1026;
    pub const ALREADY_PAUSED: u32             = 1027;
    pub const NOT_PAUSED: u32                  = 1028;
    pub const MEMBER_ALREADY_EXITED: u32       = 1029;
    pub const EXIT_REQUEST_PENDING: u32        = 1030;
    pub const NO_EXIT_REQUEST_FOUND: u32       = 1031;
    pub const EXIT_NOT_ALLOWED_MID_ROUND: u32  = 1032;
    pub const CONTRIBUTION_WINDOW_CLOSED: u32  = 1033;
    pub const FEE_EXCEEDS_MAXIMUM: u32         = 1034;
    pub const INVALID_MAX_DEFAULTS: u32        = 1035;
    pub const GROUP_FULL: u32                  = 1036;
    pub const INVALID_MAX_MEMBERS: u32         = 1037;
    pub const DELEGATION_ALREADY_EXISTS: u32   = 1038;
    pub const NO_DELEGATION_FOUND: u32         = 1039;
    pub const CANNOT_VOTE_WITH_ACTIVE_DELEGATION: u32 = 1040;
    pub const CANNOT_SUB_DELEGATE: u32         = 1041;
    pub const INVITE_NOT_FOUND: u32            = 1042;
    pub const INVITE_ALREADY_REDEEMED: u32     = 1043;
    pub const INVITE_WRONG_RECIPIENT: u32      = 1044;
    pub const ADMIN_ACTION_NOT_FOUND: u32      = 1045;
    pub const ADMIN_ACTION_ALREADY_EXECUTED: u32 = 1046;
    pub const ADMIN_ACTION_EXPIRED: u32        = 1047;
    pub const ADMIN_ALREADY_APPROVED: u32      = 1048;
    pub const INSUFFICIENT_APPROVALS: u32      = 1049;
    pub const NOT_A_CO_ADMIN: u32             = 1050;
    // ExtError variants
    pub const INVALID_TIER: u32               = 1051;
    pub const INSURANCE_POOL_NEGATIVE: u32    = 1052;
    pub const INVALID_INSURANCE_CONTRIBUTION: u32 = 1053;
    pub const SKIP_LIMIT_REACHED: u32         = 1054;
    pub const ALREADY_SKIPPED: u32            = 1055;
    pub const INSUFFICIENT_WEIGHT: u32        = 1056;
    pub const EMERGENCY_PAYOUT_REQUESTED: u32 = 1057;
    pub const EMERGENCY_PAYOUT_QUORUM_NOT_MET: u32 = 1058;
    pub const EMERGENCY_PAYOUT_VOTE_EXPIRED: u32 = 1059;
    pub const EMERGENCY_PAYOUT_ALREADY_EXECUTED: u32 = 1060;
    pub const EMERGENCY_PAYOUT_LIMIT_REACHED: u32 = 1061;
    pub const GROUP_ALREADY_DISSOLVED: u32    = 1062;
    pub const DISSOLUTION_VOTE_IN_PROGRESS: u32 = 1063;
    pub const DISSOLUTION_QUORUM_NOT_MET: u32 = 1064;
    pub const DISSOLUTION_VOTE_EXPIRED: u32   = 1065;
    pub const NO_FUNDS_TO_DISTRIBUTE: u32     = 1066;
    pub const INVALID_EMERGENCY_CONFIG: u32   = 1067;
    pub const INVALID_DISSOLUTION_CONFIG: u32 = 1068;
    pub const GROUP_NOT_YET_ACTIVE: u32       = 1069;
    pub const ONLY_ADMIN_ALLOWED: u32         = 1070;
    pub const INVALID_AMOUNT: u32             = 1071;
    pub const CO_SIGNER_ALREADY_SET: u32      = 1072;
    pub const NO_CO_SIGNER_FOUND: u32         = 1073;
    pub const CO_SIGNER_NOT_ACCEPTED: u32     = 1074;
    pub const NOT_THE_CO_SIGNER: u32          = 1075;
    pub const CO_SIGNER_WINDOW_NOT_OPEN: u32  = 1076;
    pub const CO_SIGNER_WINDOW_EXPIRED: u32   = 1077;
    pub const GROUP_FROZEN: u32               = 1078;
    pub const GROUP_NOT_FROZEN: u32           = 1079;
    pub const SNAPSHOT_TOO_SOON: u32          = 1080;
    pub const TIER_NOT_FOUND: u32             = 1081;
    pub const INVALID_TIER_DEFINITION: u32    = 1082;
    pub const INSUFFICIENT_CREDIT_SCORE: u32  = 1083;
    pub const ROUND_DURATION_OUT_OF_BOUNDS: u32 = 1084;
    pub const DELEGATION_EXPIRED: u32         = 1085;
    pub const NOT_CONTRIB_DELEGATE: u32       = 1086;
    pub const SPLIT_PROPOSAL_NOT_FOUND: u32   = 1087;
    pub const SPLIT_MEMBERS_INVALID: u32      = 1088;
    pub const SPLIT_CONFIRMATION_WINDOW_CLOSED: u32 = 1089;
    pub const SOURCE_GROUP_ALREADY_SPLIT: u32 = 1090;
    pub const SPLIT_ALREADY_CONFIRMED: u32    = 1091;
    pub const SPLIT_NOT_FULLY_CONFIRMED: u32  = 1092;
    // ExtError2 variants
    pub const AUCTION_NOT_ENABLED: u32        = 1101;
    pub const AUCTION_NOT_OPEN: u32           = 1102;
    pub const AUCTION_WINDOW_CLOSED: u32      = 1103;
    pub const INCORRECT_CONTRIBUTION_AMOUNT: u32 = 1104;
    pub const INVALID_SLOT_INDEX: u32         = 1105;
    pub const MIGRATION_ALREADY_EXECUTED: u32 = 1106;
    pub const MIGRATION_ALREADY_PENDING: u32  = 1107;
    pub const MIGRATION_NOT_APPROVED: u32     = 1108;
    pub const MIGRATION_NOT_FOUND: u32        = 1109;
    pub const NO_BID_FOUND: u32              = 1110;
    pub const SLOT_OCCUPIED: u32              = 1111;
    pub const TOKEN_MISMATCH: u32             = 1112;
    pub const OUTSTANDING_LOAN_EXISTS: u32    = 1113;
    pub const NO_COPAYERS_REGISTERED: u32     = 1114;
    pub const COPAYER_AMOUNTS_MISMATCH: u32   = 1115;
    pub const RECEIPT_NOT_FOUND: u32          = 1116;
    pub const COPAYER_SPLITS_ALREADY_SET: u32 = 1117;
    pub const PROXY_ROUNDS_EXHAUSTED: u32     = 1118;
}

// ---------------------------------------------------------------------------
// ahjoor-payments (2000–2299)
// ---------------------------------------------------------------------------

pub mod payments {
    /// See [`crate::rosca::COUNT`] for why this exists.
    pub const COUNT: usize = 47;

    pub const RATE_LIMIT_EXCEEDED: u32              = 2001;
    pub const SUBSCRIPTION_PAUSED: u32              = 2002;
    pub const ORACLE_CONDITION_NOT_MET: u32         = 2003;
    pub const SUBSCRIPTION_IN_TRIAL: u32            = 2004;
    pub const TOKEN_NOT_ALLOWED: u32                = 2005;
    pub const DUPLICATE_EXTERNAL_ID: u32            = 2006;
    pub const MULTISIG_NOT_REQUIRED: u32            = 2007;
    pub const ALREADY_APPROVED: u32                 = 2008;
    pub const NOT_A_SIGNER: u32                     = 2009;
    pub const VOUCHER_EXPIRED: u32                  = 2010;
    pub const VOUCHER_EXHAUSTED: u32                = 2011;
    pub const VOUCHER_REVOKED: u32                  = 2012;
    pub const VOUCHER_NOT_FOUND: u32                = 2013;
    pub const WITHDRAWAL_RATE_LIMIT_EXCEEDED: u32   = 2014;
    pub const REFERRAL_ALREADY_EXISTS: u32          = 2015;
    pub const NO_COMMISSION_TO_CLAIM: u32           = 2016;
    pub const DYNAMIC_PAYMENT_EXPIRED: u32          = 2017;
    pub const TIPPING_NOT_ENABLED: u32              = 2018;
    pub const TIP_EXCEEDS_MAX_BPS: u32              = 2019;
    pub const MERCHANT_VOLUME_CAPPED: u32           = 2020;
    pub const SLIPPAGE_EXCEEDED: u32                = 2021;
    pub const ORACLE_NOT_WHITELISTED: u32           = 2022;
    pub const CUSTOMER_SPEND_LIMIT_EXCEEDED: u32    = 2023;
    pub const CAPTURE_PAST_DEADLINE: u32            = 2024;
    pub const EVIDENCE_WINDOW_CLOSED: u32           = 2025;
    pub const EVIDENCE_LIMIT_REACHED: u32           = 2026;
    pub const COOLING_OFF_EXPIRED: u32              = 2027;
    pub const NOT_IN_COOLING_OFF: u32               = 2028;
    pub const COOLING_OFF_EXCEEDS_MAX: u32          = 2029;
    pub const PAUSE_COUNT_EXCEEDED: u32             = 2030;
    pub const UNAUTHORIZED_PAUSE: u32               = 2031;
    pub const INSUFFICIENT_MERCHANT_RESERVE: u32    = 2032;
    pub const KYB_VERIFICATION_REQUIRED: u32        = 2033;
    pub const RETRY_NOT_DUE: u32                    = 2034;
    pub const DEBIT_RECORD_NOT_FOUND: u32           = 2035;
    pub const DEBIT_ALREADY_ABANDONED: u32          = 2036;
    pub const DEBIT_ALREADY_SUCCEEDED: u32          = 2037;
    pub const INVALID_PAYMENT_STATUS: u32           = 2038;
    pub const MAX_EXTENSIONS_REACHED: u32           = 2039;
    pub const MAX_EXTENSION_LEDGERS_EXCEEDED: u32   = 2040;
    pub const CUSTOMER_BLOCKED: u32                 = 2050;
    pub const DAO_NOT_CONFIGURED: u32               = 2051;
    pub const NOT_A_DAO_MEMBER: u32                 = 2052;
    pub const DAO_ALREADY_ESCALATED: u32            = 2053;
    pub const DAO_VOTE_WINDOW_OPEN: u32             = 2054;
    pub const DAO_VOTE_WINDOW_CLOSED: u32           = 2055;
    pub const DAO_ALREADY_VOTED: u32                = 2056;
}

// ---------------------------------------------------------------------------
// ahjoor-escrow (3000–3299)
// ---------------------------------------------------------------------------

pub mod escrow {
    /// See [`crate::rosca::COUNT`] for why this exists.
    pub const COUNT: usize = 3;

    pub const INVALID_DEADLINE: u32        = 3001;
    pub const INVALID_TRANCHE_INDEX: u32   = 3002;
    pub const TRANCHE_ALREADY_CLAIMED: u32 = 3003;
}

// ---------------------------------------------------------------------------
// ahjoor-refund (4000–4099)
// ---------------------------------------------------------------------------

pub mod refund {
    /// See [`crate::rosca::COUNT`] for why this exists.
    pub const COUNT: usize = 8;

    // Refund contract uses panic! rather than a contracterror enum;
    // these codes are the off-chain namespace assignments for future migration.
    pub const ALREADY_INITIALIZED: u32             = 4001;
    pub const FEE_EXCEEDS_MAXIMUM: u32             = 4002;
    pub const AMOUNT_MUST_BE_POSITIVE: u32         = 4003;
    pub const INVALID_REASON_CODE: u32             = 4004;
    pub const REFUND_COOLDOWN_ACTIVE: u32          = 4005;
    pub const PAYMENT_NOT_FOUND: u32               = 4006;
    pub const PAYMENT_NOT_COMPLETED: u32           = 4007;
    pub const EXCEEDS_REFUNDABLE_AMOUNT: u32       = 4008;
}

// ---------------------------------------------------------------------------
// ahjoor-token-whitelist (5000–5099)
// ---------------------------------------------------------------------------

pub mod whitelist {
    /// See [`crate::rosca::COUNT`] for why this exists.
    pub const COUNT: usize = 8;

    pub const NOT_INITIALIZED: u32            = 5001;
    pub const ALREADY_INITIALIZED: u32        = 5002;
    pub const UNAUTHORIZED: u32               = 5003;
    pub const TOKEN_ALREADY_WHITELISTED: u32  = 5004;
    pub const TOKEN_NOT_WHITELISTED: u32      = 5005;
    pub const QUOTA_EXCEEDED: u32             = 5006;
    pub const TOKEN_ALREADY_HAS_QUOTA: u32    = 5007;
    pub const TOKEN_HAS_NO_QUOTA: u32         = 5008;
}

// ---------------------------------------------------------------------------
// Convenience: machine-readable error descriptor
// ---------------------------------------------------------------------------

/// Compact descriptor for one error code entry (used in errors.json generation).
pub struct ErrorEntry {
    pub code: u32,
    pub name: &'static str,
    pub contract: &'static str,
}

pub static ALL_ERRORS: &[ErrorEntry] = &[
    // rosca (110 entries — must match rosca::COUNT)
    ErrorEntry { code: rosca::ALREADY_INITIALIZED, name: "AlreadyInitialized", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::TOKEN_NOT_APPROVED, name: "TokenNotApproved", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CUSTOM_ORDER_LENGTH_MISMATCH, name: "CustomOrderLengthMismatch", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CUSTOM_ORDER_NON_MEMBER, name: "CustomOrderNonMember", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::AMOUNT_MUST_BE_POSITIVE, name: "AmountMustBePositive", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ROUND_DEADLINE_PASSED, name: "RoundDeadlinePassed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MEMBER_HAS_EXITED, name: "MemberHasExited", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_A_MEMBER, name: "NotAMember", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALREADY_CONTRIBUTED, name: "AlreadyContributed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_EXCHANGE_RATE, name: "InvalidExchangeRate", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EXCEEDS_TOKEN_LIMIT, name: "ExceedsTokenLimit", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EXCEEDS_REMAINING_CONTRIBUTION, name: "ExceedsRemainingContribution", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DEADLINE_NOT_PASSED, name: "DeadlineNotPassed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::PENALTY_DISABLED, name: "PenaltyDisabled", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_A_DEFAULTER, name: "NotADefaulter", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CANNOT_CHANGE_MID_ROUND, name: "CannotChangeMidRound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALREADY_A_MEMBER, name: "AlreadyAMember", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_REWARDS_TO_CLAIM, name: "NoRewardsToClaim", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ONLY_MEMBERS_ALLOWED, name: "OnlyMembersAllowed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::PROPOSAL_NOT_FOUND, name: "ProposalNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::VOTING_DEADLINE_PASSED, name: "VotingDeadlinePassed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::PROPOSAL_NOT_PENDING, name: "ProposalNotPending", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALREADY_VOTED, name: "AlreadyVoted", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::VOTING_NOT_ENDED, name: "VotingNotEnded", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CONTRACT_PAUSED, name: "ContractPaused", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALL_MEMBERS_SUSPENDED, name: "AllMembersSuspended", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALREADY_PAUSED, name: "AlreadyPaused", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_PAUSED, name: "NotPaused", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MEMBER_ALREADY_EXITED, name: "MemberAlreadyExited", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EXIT_REQUEST_PENDING, name: "ExitRequestPending", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_EXIT_REQUEST_FOUND, name: "NoExitRequestFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EXIT_NOT_ALLOWED_MID_ROUND, name: "ExitNotAllowedMidRound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CONTRIBUTION_WINDOW_CLOSED, name: "ContributionWindowClosed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::FEE_EXCEEDS_MAXIMUM, name: "FeeExceedsMaximum", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_MAX_DEFAULTS, name: "InvalidMaxDefaults", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::GROUP_FULL, name: "GroupFull", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_MAX_MEMBERS, name: "InvalidMaxMembers", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DELEGATION_ALREADY_EXISTS, name: "DelegationAlreadyExists", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_DELEGATION_FOUND, name: "NoDelegationFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CANNOT_VOTE_WITH_ACTIVE_DELEGATION, name: "CannotVoteWithActiveDelegation", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CANNOT_SUB_DELEGATE, name: "CannotSubDelegate", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVITE_NOT_FOUND, name: "InviteNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVITE_ALREADY_REDEEMED, name: "InviteAlreadyRedeemed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVITE_WRONG_RECIPIENT, name: "InviteWrongRecipient", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ADMIN_ACTION_NOT_FOUND, name: "AdminActionNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ADMIN_ACTION_ALREADY_EXECUTED, name: "AdminActionAlreadyExecuted", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ADMIN_ACTION_EXPIRED, name: "AdminActionExpired", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ADMIN_ALREADY_APPROVED, name: "AdminAlreadyApproved", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INSUFFICIENT_APPROVALS, name: "InsufficientApprovals", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_A_CO_ADMIN, name: "NotACoAdmin", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_TIER, name: "InvalidTier", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INSURANCE_POOL_NEGATIVE, name: "InsurancePoolNegative", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_INSURANCE_CONTRIBUTION, name: "InvalidInsuranceContribution", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SKIP_LIMIT_REACHED, name: "SkipLimitReached", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ALREADY_SKIPPED, name: "AlreadySkipped", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INSUFFICIENT_WEIGHT, name: "InsufficientWeight", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EMERGENCY_PAYOUT_REQUESTED, name: "EmergencyPayoutRequested", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EMERGENCY_PAYOUT_QUORUM_NOT_MET, name: "EmergencyPayoutQuorumNotMet", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EMERGENCY_PAYOUT_VOTE_EXPIRED, name: "EmergencyPayoutVoteExpired", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EMERGENCY_PAYOUT_ALREADY_EXECUTED, name: "EmergencyPayoutAlreadyExecuted", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::EMERGENCY_PAYOUT_LIMIT_REACHED, name: "EmergencyPayoutLimitReached", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::GROUP_ALREADY_DISSOLVED, name: "GroupAlreadyDissolved", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DISSOLUTION_VOTE_IN_PROGRESS, name: "DissolutionVoteInProgress", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DISSOLUTION_QUORUM_NOT_MET, name: "DissolutionQuorumNotMet", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DISSOLUTION_VOTE_EXPIRED, name: "DissolutionVoteExpired", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_FUNDS_TO_DISTRIBUTE, name: "NoFundsToDistribute", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_EMERGENCY_CONFIG, name: "InvalidEmergencyConfig", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_DISSOLUTION_CONFIG, name: "InvalidDissolutionConfig", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::GROUP_NOT_YET_ACTIVE, name: "GroupNotYetActive", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ONLY_ADMIN_ALLOWED, name: "OnlyAdminAllowed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_AMOUNT, name: "InvalidAmount", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CO_SIGNER_ALREADY_SET, name: "CoSignerAlreadySet", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_CO_SIGNER_FOUND, name: "NoCoSignerFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CO_SIGNER_NOT_ACCEPTED, name: "CoSignerNotAccepted", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_THE_CO_SIGNER, name: "NotTheCoSigner", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CO_SIGNER_WINDOW_NOT_OPEN, name: "CoSignerWindowNotOpen", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::CO_SIGNER_WINDOW_EXPIRED, name: "CoSignerWindowExpired", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::GROUP_FROZEN, name: "GroupFrozen", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::GROUP_NOT_FROZEN, name: "GroupNotFrozen", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SNAPSHOT_TOO_SOON, name: "SnapshotTooSoon", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::TIER_NOT_FOUND, name: "TierNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_TIER_DEFINITION, name: "InvalidTierDefinition", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INSUFFICIENT_CREDIT_SCORE, name: "InsufficientCreditScore", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::ROUND_DURATION_OUT_OF_BOUNDS, name: "RoundDurationOutOfBounds", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::DELEGATION_EXPIRED, name: "DelegationExpired", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NOT_CONTRIB_DELEGATE, name: "NotContribDelegate", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SPLIT_PROPOSAL_NOT_FOUND, name: "SplitProposalNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SPLIT_MEMBERS_INVALID, name: "SplitMembersInvalid", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SPLIT_CONFIRMATION_WINDOW_CLOSED, name: "SplitConfirmationWindowClosed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SOURCE_GROUP_ALREADY_SPLIT, name: "SourceGroupAlreadySplit", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SPLIT_ALREADY_CONFIRMED, name: "SplitAlreadyConfirmed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SPLIT_NOT_FULLY_CONFIRMED, name: "SplitNotFullyConfirmed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::AUCTION_NOT_ENABLED, name: "AuctionNotEnabled", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::AUCTION_NOT_OPEN, name: "AuctionNotOpen", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::AUCTION_WINDOW_CLOSED, name: "AuctionWindowClosed", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INCORRECT_CONTRIBUTION_AMOUNT, name: "IncorrectContributionAmount", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::INVALID_SLOT_INDEX, name: "InvalidSlotIndex", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MIGRATION_ALREADY_EXECUTED, name: "MigrationAlreadyExecuted", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MIGRATION_ALREADY_PENDING, name: "MigrationAlreadyPending", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MIGRATION_NOT_APPROVED, name: "MigrationNotApproved", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::MIGRATION_NOT_FOUND, name: "MigrationNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_BID_FOUND, name: "NoBidFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::SLOT_OCCUPIED, name: "SlotOccupied", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::TOKEN_MISMATCH, name: "TokenMismatch", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::OUTSTANDING_LOAN_EXISTS, name: "OutstandingLoanExists", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::NO_COPAYERS_REGISTERED, name: "NoCopayersRegistered", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::COPAYER_AMOUNTS_MISMATCH, name: "CopayerAmountsMismatch", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::RECEIPT_NOT_FOUND, name: "ReceiptNotFound", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::COPAYER_SPLITS_ALREADY_SET, name: "CopayerSplitsAlreadySet", contract: "ahjoor-rosca" },
    ErrorEntry { code: rosca::PROXY_ROUNDS_EXHAUSTED, name: "ProxyRoundsExhausted", contract: "ahjoor-rosca" },

    // payments (47 entries — must match payments::COUNT)
    ErrorEntry { code: payments::RATE_LIMIT_EXCEEDED, name: "RateLimitExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::SUBSCRIPTION_PAUSED, name: "SubscriptionPaused", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::ORACLE_CONDITION_NOT_MET, name: "OracleConditionNotMet", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::SUBSCRIPTION_IN_TRIAL, name: "SubscriptionInTrial", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::TOKEN_NOT_ALLOWED, name: "TokenNotAllowed", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DUPLICATE_EXTERNAL_ID, name: "DuplicateExternalId", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::MULTISIG_NOT_REQUIRED, name: "MultisigNotRequired", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::ALREADY_APPROVED, name: "AlreadyApproved", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::NOT_A_SIGNER, name: "NotASigner", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::VOUCHER_EXPIRED, name: "VoucherExpired", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::VOUCHER_EXHAUSTED, name: "VoucherExhausted", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::VOUCHER_REVOKED, name: "VoucherRevoked", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::VOUCHER_NOT_FOUND, name: "VoucherNotFound", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::WITHDRAWAL_RATE_LIMIT_EXCEEDED, name: "WithdrawalRateLimitExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::REFERRAL_ALREADY_EXISTS, name: "ReferralAlreadyExists", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::NO_COMMISSION_TO_CLAIM, name: "NoCommissionToClaim", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DYNAMIC_PAYMENT_EXPIRED, name: "DynamicPaymentExpired", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::TIPPING_NOT_ENABLED, name: "TippingNotEnabled", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::TIP_EXCEEDS_MAX_BPS, name: "TipExceedsMaxBps", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::MERCHANT_VOLUME_CAPPED, name: "MerchantVolumeCapped", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::SLIPPAGE_EXCEEDED, name: "SlippageExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::ORACLE_NOT_WHITELISTED, name: "OracleNotWhitelisted", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::CUSTOMER_SPEND_LIMIT_EXCEEDED, name: "CustomerSpendLimitExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::CAPTURE_PAST_DEADLINE, name: "CapturePastDeadline", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::EVIDENCE_WINDOW_CLOSED, name: "EvidenceWindowClosed", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::EVIDENCE_LIMIT_REACHED, name: "EvidenceLimitReached", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::COOLING_OFF_EXPIRED, name: "CoolingOffExpired", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::NOT_IN_COOLING_OFF, name: "NotInCoolingOff", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::COOLING_OFF_EXCEEDS_MAX, name: "CoolingOffExceedsMax", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::PAUSE_COUNT_EXCEEDED, name: "PauseCountExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::UNAUTHORIZED_PAUSE, name: "UnauthorizedPause", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::INSUFFICIENT_MERCHANT_RESERVE, name: "InsufficientMerchantReserve", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::KYB_VERIFICATION_REQUIRED, name: "KYBVerificationRequired", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::RETRY_NOT_DUE, name: "RetryNotDue", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DEBIT_RECORD_NOT_FOUND, name: "DebitRecordNotFound", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DEBIT_ALREADY_ABANDONED, name: "DebitAlreadyAbandoned", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DEBIT_ALREADY_SUCCEEDED, name: "DebitAlreadySucceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::INVALID_PAYMENT_STATUS, name: "InvalidPaymentStatus", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::MAX_EXTENSIONS_REACHED, name: "MaxExtensionsReached", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::MAX_EXTENSION_LEDGERS_EXCEEDED, name: "MaxExtensionLedgersExceeded", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::CUSTOMER_BLOCKED, name: "CustomerBlocked", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DAO_NOT_CONFIGURED, name: "DaoNotConfigured", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::NOT_A_DAO_MEMBER, name: "NotADaoMember", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DAO_ALREADY_ESCALATED, name: "DaoAlreadyEscalated", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DAO_VOTE_WINDOW_OPEN, name: "DaoVoteWindowOpen", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DAO_VOTE_WINDOW_CLOSED, name: "DaoVoteWindowClosed", contract: "ahjoor-payments" },
    ErrorEntry { code: payments::DAO_ALREADY_VOTED, name: "DaoAlreadyVoted", contract: "ahjoor-payments" },

    // escrow (3 entries — must match escrow::COUNT)
    ErrorEntry { code: escrow::INVALID_DEADLINE, name: "InvalidDeadline", contract: "ahjoor-escrow" },
    ErrorEntry { code: escrow::INVALID_TRANCHE_INDEX, name: "InvalidTrancheIndex", contract: "ahjoor-escrow" },
    ErrorEntry { code: escrow::TRANCHE_ALREADY_CLAIMED, name: "TrancheAlreadyClaimed", contract: "ahjoor-escrow" },

    // refund (8 entries — must match refund::COUNT)
    ErrorEntry { code: refund::ALREADY_INITIALIZED, name: "AlreadyInitialized", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::FEE_EXCEEDS_MAXIMUM, name: "FeeExceedsMaximum", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::AMOUNT_MUST_BE_POSITIVE, name: "AmountMustBePositive", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::INVALID_REASON_CODE, name: "InvalidReasonCode", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::REFUND_COOLDOWN_ACTIVE, name: "RefundCooldownActive", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::PAYMENT_NOT_FOUND, name: "PaymentNotFound", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::PAYMENT_NOT_COMPLETED, name: "PaymentNotCompleted", contract: "ahjoor-refund" },
    ErrorEntry { code: refund::EXCEEDS_REFUNDABLE_AMOUNT, name: "ExceedsRefundableAmount", contract: "ahjoor-refund" },

    // whitelist (8 entries — must match whitelist::COUNT)
    ErrorEntry { code: whitelist::NOT_INITIALIZED, name: "NotInitialized", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::ALREADY_INITIALIZED, name: "AlreadyInitialized", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::UNAUTHORIZED, name: "Unauthorized", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::TOKEN_ALREADY_WHITELISTED, name: "TokenAlreadyWhitelisted", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::TOKEN_NOT_WHITELISTED, name: "TokenNotWhitelisted", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::QUOTA_EXCEEDED, name: "QuotaExceeded", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::TOKEN_ALREADY_HAS_QUOTA, name: "TokenAlreadyHasQuota", contract: "ahjoor-token-whitelist" },
    ErrorEntry { code: whitelist::TOKEN_HAS_NO_QUOTA, name: "TokenHasNoQuota", contract: "ahjoor-token-whitelist" },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicate_codes() {
        let mut seen = std::vec::Vec::new();
        for entry in ALL_ERRORS {
            assert!(
                !seen.contains(&entry.code),
                "Duplicate error code {} ({}::{})",
                entry.code,
                entry.contract,
                entry.name,
            );
            seen.push(entry.code);
        }
    }

    /// Guards against the exact failure mode that motivated this registry:
    /// a `pub const` added to a module without a matching `ALL_ERRORS` entry
    /// (or an entry added to the wrong contract/range). Each module exposes
    /// a `COUNT` const alongside its error codes; this test cross-checks
    /// that count against how many `ALL_ERRORS` entries are tagged for that
    /// contract. If someone adds a new error code, they must also bump the
    /// module's `COUNT` and add an `ALL_ERRORS` entry — if either step is
    /// skipped, this test fails.
    #[test]
    fn all_errors_covers_every_module_const() {
        let expected: &[(&str, usize)] = &[
            ("ahjoor-rosca", rosca::COUNT),
            ("ahjoor-payments", payments::COUNT),
            ("ahjoor-escrow", escrow::COUNT),
            ("ahjoor-refund", refund::COUNT),
            ("ahjoor-token-whitelist", whitelist::COUNT),
        ];

        for (contract, count) in expected {
            let actual = ALL_ERRORS.iter().filter(|e| &e.contract == contract).count();
            assert_eq!(
                actual, *count,
                "{contract}: ALL_ERRORS has {actual} entries but the module declares COUNT = {count}. \
                 A const was added/removed without updating the other.",
            );
        }

        let total_expected: usize = expected.iter().map(|(_, c)| c).sum();
        assert_eq!(
            ALL_ERRORS.len(),
            total_expected,
            "ALL_ERRORS contains entries for a contract not covered by this test",
        );
    }

    #[test]
    fn codes_within_contract_ranges() {
        for entry in ALL_ERRORS {
            let in_range = match entry.contract {
                "ahjoor-rosca"            => (1000..=1299).contains(&entry.code),
                "ahjoor-payments"         => (2000..=2299).contains(&entry.code),
                "ahjoor-escrow"           => (3000..=3299).contains(&entry.code),
                "ahjoor-refund"           => (4000..=4099).contains(&entry.code),
                "ahjoor-token-whitelist"  => (5000..=5099).contains(&entry.code),
                _                         => false,
            };
            assert!(
                in_range,
                "Error code {} ({}) is outside the expected range for {}",
                entry.code,
                entry.name,
                entry.contract,
            );
        }
    }
}
