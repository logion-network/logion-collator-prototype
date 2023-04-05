#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
pub mod xcm_config;

use logion_shared::{CreateRecoveryCallFactory, MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory, DistributionKey};
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, One, Verify},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, MultiSignature, Percent,
};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{ConstU32, ConstU64, ConstU8, Contains, Currency, Imbalance, OnUnbalanced},
	weights::{
		constants::WEIGHT_REF_TIME_PER_SECOND, ConstantMultiplier, Weight,
	},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use pallet_multisig::Timepoint;
use pallet_transaction_payment::{CurrencyAdapter, Multiplier};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot imports
use polkadot_runtime_common::{BlockHashCount, SlowAdjustingFeeUpdate};

use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

// XCM Imports
use xcm::latest::prelude::BodyId;
use xcm_executor::XcmExecutor;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// LOC ID
pub type LocId = u128;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{generic, traits::BlakeTwo256};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("logion"),
	impl_name: create_runtime_str!("logion"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000_000_000;
pub const MICROUNIT: Balance = 1_000_000_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

pub const NANO_LGNT: Balance = 1_000_000_000;
pub const MICRO_LGNT: Balance = 1_000 * NANO_LGNT;
pub const MILLI_LGNT: Balance = 1_000 * MICRO_LGNT;
pub const LGNT: Balance = 1_000 * MILLI_LGNT;

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{..}) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{..}) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::as_multi{..}) => false,
			_ => true
		}
	}
}

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseCallFilter;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	/// The maximum number of consumers allowed on a single account.
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub TreasuryAccountId: AccountId = TreasuryPalletId::get().into_account_truncating();
	pub const InclusionFeesToBurnPercent: u32 = 100;
	pub const InclusionFeesTreasuryPercent: u32 = 0; // Inclusion fees disabled for the moment
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct DealWithInclusionFees;

impl OnUnbalanced<NegativeImbalance> for DealWithInclusionFees {

	fn on_nonzero_unbalanced(fees: NegativeImbalance) {

		let (to_burn, treasury) = fees.ration(InclusionFeesToBurnPercent::get(), InclusionFeesTreasuryPercent::get());
		drop(to_burn);
		if treasury != NegativeImbalance::zero() {
			Balances::resolve_creating(&TreasuryPalletId::get().into_account_truncating(), treasury);
		}
	}
}

pub type WeightToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
	pub const WeightToFeeMultiplier: Balance = 10_000_000;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithInclusionFees>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
	type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but let's be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const MinCandidates: u32 = 5;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinCandidates = MinCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

parameter_types! {
	pub const RecoveryConfigDepositBase: u64 = 10;
	pub const RecoveryFrieldDepositFactor: u64 = 1;
	pub const MaxFriends: u16 = 3;
	pub const RecoveryDeposit: u64 = 10;
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = RecoveryConfigDepositBase;
	type FriendDepositFactor = RecoveryFrieldDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
	type WeightInfo = ();
}

impl pallet_lo_authority_list::Config for Runtime {
	type AddOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const MaxMetadataItemNameSize: usize = 255;
	pub const MaxMetadataItemValueSize: usize = 4096;
	pub const MaxFileNatureSize: usize = 255;
	pub const MaxLinkNatureSize: usize = 255;
	pub const MaxCollectionItemDescriptionSize: usize = 4096;
	pub const MaxCollectionItemTokenIdSize: usize = 255;
	pub const MaxCollectionItemTokenTypeSize: usize = 255;
	pub const MaxFileContentTypeSize: u32 = 255;
	pub const MaxFileNameSize: u32 = 255;
	pub const MaxTokensRecordDescriptionSize: u32 = 4096;
	pub const MaxTokensRecordFiles: u32 = 10;
	pub const FileStorageByteFee: Balance = 100 * NANO_LGNT; // 0.1 LGNT per MB
	pub const FileStorageEntryFee: Balance = 0;
    pub const FileStorageFeeDistributionKey: DistributionKey = DistributionKey {
        stakers_percent: Percent::from_percent(0),
        collators_percent: Percent::from_percent(20),
        reserve_percent: Percent::from_percent(80),
    };
}

impl pallet_logion_loc::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type Hash = Hash;
	type IsLegalOfficer = LoAuthorityList;
	type MaxMetadataItemNameSize = MaxMetadataItemNameSize;
	type MaxMetadataItemValueSize = MaxMetadataItemValueSize;
	type MaxFileNatureSize = MaxFileNatureSize;
	type MaxLinkNatureSize = MaxLinkNatureSize;
	type CollectionItemId = Hash;
	type MaxCollectionItemDescriptionSize = MaxCollectionItemDescriptionSize;
	type MaxCollectionItemTokenIdSize = MaxCollectionItemTokenIdSize;
	type MaxCollectionItemTokenTypeSize = MaxCollectionItemTokenTypeSize;
	type TokensRecordId = Hash;
	type MaxFileContentTypeSize = MaxFileContentTypeSize;
	type MaxFileNameSize = MaxFileNameSize;
	type MaxTokensRecordDescriptionSize = MaxTokensRecordDescriptionSize;
	type MaxTokensRecordFiles = MaxTokensRecordFiles;
	type WeightInfo = ();
	type Currency = Balances;
	type FileStorageByteFee = FileStorageByteFee;
	type FileStorageEntryFee = FileStorageEntryFee;
	type FileStorageFeeDistributor = RewardDistributor;
	type FileStorageFeeDistributionKey = FileStorageFeeDistributionKey;
}

pub struct PalletRecoveryCreateRecoveryCallFactory;
impl CreateRecoveryCallFactory<RuntimeOrigin, AccountId, BlockNumber> for PalletRecoveryCreateRecoveryCallFactory {
	type Call = RuntimeCall;

	fn build_create_recovery_call(legal_officers: Vec<AccountId>, threshold: u16, delay_period: BlockNumber) -> RuntimeCall {
		RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{ friends : legal_officers, threshold, delay_period })
	}
}

impl pallet_verified_recovery::Config for Runtime {
	type LocId = LocId;
	type CreateRecoveryCallFactory = PalletRecoveryCreateRecoveryCallFactory;
	type LocQuery = LogionLoc;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const MultiSigDepositBase: Balance = 500;
	pub const MultiSigDepositFactor: Balance = 100;
	pub const MaxSignatories: u16 = 20;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = MultiSigDepositBase;
	type DepositFactor = MultiSigDepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = ();
}

pub struct PalletMultisigApproveAsMultiCallFactory;
impl MultisigApproveAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigApproveAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_approve_as_multi_call(
        threshold: u16,
        other_signatories: Vec<AccountId>,
        maybe_timepoint: Option<Timepoint<BlockNumber>>,
        call_hash: [u8; 32],
        max_weight: Weight
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{ threshold, other_signatories, maybe_timepoint, call_hash, max_weight })
	}
}

pub struct PalletMultisigAsMultiCallFactory;
impl MultisigAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_as_multi_call(
        threshold: u16,
        other_signatories: Vec<AccountId>,
        maybe_timepoint: Option<Timepoint<BlockNumber>>,
        call: Box<Self::Call>,
        max_weight: Weight,
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::as_multi{ threshold, other_signatories, maybe_timepoint, call, max_weight })
	}
}

impl pallet_logion_vault::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type MultisigApproveAsMultiCallFactory = PalletMultisigApproveAsMultiCallFactory;
	type MultisigAsMultiCallFactory = PalletMultisigAsMultiCallFactory;
	type IsLegalOfficer = LoAuthorityList;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_logion_vote::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type IsLegalOfficer = LoAuthorityList;
	type LocValidity = LogionLoc;
	type LocQuery = LogionLoc;
	type LegalOfficerCreation = LoAuthorityList;
}

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 100 * LGNT;
    pub const SpendPeriod: BlockNumber = 1 * DAYS;
}

impl pallet_treasury::Config for Runtime {
	type Currency = Balances;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = ();
	type PalletId = TreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	type SpendFunds = ();
	type MaxApprovals = ConstU32<100>;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

pub const BLOCK_REWARD: Balance = 0; // Inflation disabled for the moment

parameter_types! {
    pub const RewardAmount: Balance = BLOCK_REWARD;
    pub const RewardDistributionKey: DistributionKey = DistributionKey {
        stakers_percent: Percent::from_percent(50),
        collators_percent: Percent::from_percent(30),
        reserve_percent: Percent::from_percent(20),
    };
}

pub struct RewardDistributor();
impl logion_shared::RewardDistributor<NegativeImbalance, Balance>
    for RewardDistributor
{
    fn payout_reserve(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&TreasuryPalletId::get().into_account_truncating(), reward);
		}
    }

    fn payout_collators(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&TreasuryPalletId::get().into_account_truncating(), reward);
		}
    }

    fn payout_stakers(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&TreasuryPalletId::get().into_account_truncating(), reward);
		}
    }
}

impl pallet_block_reward::Config for Runtime {
    type Currency = Balances;
    type RewardAmount = RewardAmount;
    type RewardDistributor = RewardDistributor;
    type DistributionKey = RewardDistributionKey;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// System support stuff.
		System: frame_system = 0,
		ParachainSystem: cumulus_pallet_parachain_system = 1,
		Timestamp: pallet_timestamp = 2,
		ParachainInfo: parachain_info = 3,

		// Monetary stuff.
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,

		// Collator support. The order of these 4 are important and shall not change.
		Authorship: pallet_authorship = 20,
		CollatorSelection: pallet_collator_selection = 21,
		Session: pallet_session = 22,
		Aura: pallet_aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 30,
		PolkadotXcm: pallet_xcm = 31,
		CumulusXcm: cumulus_pallet_xcm = 32,
		DmpQueue: cumulus_pallet_dmp_queue = 33,

		// Logion
		Sudo: pallet_sudo = 40,
		Multisig:  pallet_multisig = 41,
		Recovery: pallet_recovery = 42,
		LoAuthorityList: pallet_lo_authority_list = 43,
		LogionLoc: pallet_logion_loc = 44,
		VerifiedRecovery: pallet_verified_recovery = 45,
		Vault: pallet_logion_vault = 46,
		Vote: pallet_logion_vote = 47,
		Treasury: pallet_treasury = 48,
		BlockReward: pallet_block_reward = 49,
	}
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	frame_benchmarking::define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_session, SessionBench::<Runtime>]
		[pallet_timestamp, Timestamp]
		[pallet_collator_selection, CollatorSelection]
		[cumulus_pallet_xcmp_queue, XcmpQueue]
	);
}

impl_runtime_apis! {
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect,
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();
			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			impl cumulus_pallet_session_benchmarking::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
