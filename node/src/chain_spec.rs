use cumulus_primitives_core::ParaId;
use logion_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use pallet_lo_authority_list::{LegalOfficerData, HostData};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, OpaquePeerId, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<logion_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> logion_runtime::SessionKeys {
	logion_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![ // Initial set of Logion Legal Officers
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
							base_url: None,
						})
					),
				],
			)
		},
		Vec::new(),
		None,
		Some("logion"),
		None,
		Some(default_properties("LGNT")),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: test_parachain_id().into(),
		},
	)
}

pub fn chimay_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Chimay",
		// ID
		"chimay",
		ChainType::Live,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![ // Initial set of logion Legal Officers
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
							base_url: None,
						})
					),
				],
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("chimay"),
		// Fork ID
		None,
		// Properties
		Some(default_properties("CHY")),
		// Extensions
		Extensions {
			relay_chain: "rococo_orval".into(),
			para_id: test_parachain_id().into(),
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Logion",
		// ID
		"local_logion",
		ChainType::Local,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![ // Initial set of Logion Legal Officers
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
							base_url: None,
						})
					),
				],
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("logion"),
		// Fork ID
		None,
		// Properties
		Some(default_properties("LGNT")),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: test_parachain_id().into(),
		},
	)
}

fn build_genesis_config(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
	root_key: AccountId,
	legal_officers: Vec<(AccountId, LegalOfficerData<AccountId>)>,
) -> logion_runtime::GenesisConfig {
	logion_runtime::GenesisConfig {
		system: logion_runtime::SystemConfig {
			code: logion_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: logion_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: logion_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: logion_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: logion_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: logion_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: logion_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		lo_authority_list: logion_runtime::LoAuthorityListConfig {
			legal_officers,
		},
		transaction_payment: Default::default(),
		treasury: Default::default(),
	}
}

fn default_properties(symbol: &str) -> sc_service::Properties {
	let mut props : sc_service::Properties = sc_service::Properties::new();
	props.insert("tokenSymbol".into(), symbol.into());
	props.insert("tokenDecimals".into(), 18.into());
	return props;
}

fn test_parachain_id() -> ParaId {
	2000.into()
}
