use cumulus_primitives_core::ParaId;
use logion_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT, Balance, LGNT};
use pallet_lo_authority_list::{LegalOfficerData, HostData};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, OpaquePeerId, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

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

const DEFAULT_TEST_BALANCE: Balance = 1 << 60;

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
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
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
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
				],
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
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

pub fn main_config() -> ChainSpec {
	const ROOT_PUBLIC_SR25519: &str = "5FUg3QWfipPf8yKv5hMK6wQf8nn6og9BbRNcr3Y8CwUJwTh9";

	const NODE01_PUBLIC_SR25519: &str = "5DjzFDhFidvGCuuy6i8Lsi4XyruYjxTTkJKb1o7XzVdMNPVb";
	const NODE02_PUBLIC_SR25519: &str = "5DoD9n61SssFiWQDTD7bz1eX3KCxZJ6trVj2GsDwMi2PqP85";
	const NODE03_PUBLIC_SR25519: &str = "5CJTSSJ4v1RAauZpeqTeddyui4wESZZqPor33wum9aKuQXZC";
	const NODE04_PUBLIC_SR25519: &str = "5EF6NVgMfRRFMRnNEByNJsQJfD1fokamB9kq2J7SLRVraJrg";
	const NODE05_PUBLIC_SR25519: &str = "5G7Gtz7iLn3z5PkqfweQJp5jJdV3u8ix7qWcGS4bs38EH1W3";
	const NODE06_PUBLIC_SR25519: &str = "5EZRCd7FybQKthaD2XuV21RAdU5LqPoveiSdrz9Z6JCstoSH";
	const NODE07_PUBLIC_SR25519: &str = "5DqwojnfMTfZvERe9SJr3e1ApfaAY8Lye8Tch6WfnmxkfJfw";
	const NODE08_PUBLIC_SR25519: &str = "5GRie9PZxqzAmPoJAgiLjzgxzFi7LW2ez1TNzzWdUN6yh8Jd";
	const NODE09_PUBLIC_SR25519: &str = "5CSsbWDRbV5eYuWZsSrFcfkrEnGAjhbmyGJjjpRkjQ7s5dCd";
	const NODE10_PUBLIC_SR25519: &str = "5FYe8QZfCUZVh6BeuAziATXNcowbZuSngqrguGahscdbhhnz";
	const NODE11_PUBLIC_SR25519: &str = "5DRbgvZC3LEeJmRe893Q3UEwP2H1DPv5x8ofFgcxihCLu3oL";
	const NODE12_PUBLIC_SR25519: &str = "5F6h3kuXnhpwkVzDKRd65jrSu53UecKNRdHcgCGFiAbAPWMt";

	ChainSpec::from_genesis(
		"logion network",
		"logion",
		ChainType::Live,
		move || {
			build_genesis_config(
				vec![
					(
						AccountId::from_str(NODE01_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE01_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE02_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE02_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE03_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE03_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE04_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE04_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE05_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE05_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE06_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE06_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE07_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE07_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE08_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE08_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE09_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE09_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE10_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE10_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE11_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE11_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE12_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE12_PUBLIC_SR25519).unwrap()),
					),
				],
				vec![
					(
						AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
						10 * LGNT,
					),
				],
				main_para_id().into(),
				AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
				vec![
					(
						AccountId::from_str("5FmqTpGanDBVHedXf42fiuWD8d2iBa2Ve8EfG13juifnpgat").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWPPCrBT2WxxPuBmdMFRs1JddaZjTPWvNdgRzWoFzZw2yT").into_vec().unwrap())),
							base_url: Some("https://node01.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5GYirZEq8byGJePM9FM3JQG8Zwc5B6AcNpqgbrFvGRw2VQKE").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWSweFqPDamxmzjpgX7Q4bvfnpRKzTJ1igsYLU2ZsLL1TM").into_vec().unwrap())),
							base_url: Some("https://node02.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5FQvrVyaxF6bmQkSKb6Xr9LdiWG4sr3CoyqPQvxJusowisoj").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJSnG148nKuds3cEjYrjFMPNWh6biVBPxuppgQnn1owZC").into_vec().unwrap())),
							base_url: Some("https://node03.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5Gn9QQ6Nnut9qv3yPH2N8ZheaYGaEDQZAiRrdiDq3sBBFPQ2").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWDM1X5iuEmGvxoCjhWXZLMAh3oqfQrAPvaVS8qunKdWCD").into_vec().unwrap())),
							base_url: Some("https://node04.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5HQjQDPEZ8kxGmr5qKdURSMc18753TSH8FvL39i3Bfd5YRCa").unwrap(), // Never registered
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWF2NhEHjy8tvtsG7VxHEJaaXDhEqgwrW5Jb9N5pYd5pYv").into_vec().unwrap())),
							base_url: Some("https://node05.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5GWxv6y9XA2CQG3xZ3jDXkvYs4bucazqByHQYrjn8u8mH2qE").unwrap(), // Never registered
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWCM56Vtr8puXPbhwGzDNj66hq5zGwCy23piCfDprSfPEK").into_vec().unwrap())),
							base_url: Some("https://node06.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5CifPzNnEb8ffgoBB26dkcTKDKZnuMg7c3YyZqLKgyQQPrgB").unwrap(), // Never registered
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWRTrKaUzLeKk21QAke4ifXUo41CRvtJfsvwxYd7UWfcjU").into_vec().unwrap())),
							base_url: Some("https://node07.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5CZy9rGJBsSF9tQ6SkWsjA7kTBiN5ZYJm9zs5ByVPDHCkNHJ").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWF5rMEqrfrUbQah8RYhvUTyvumpbTeQVoib7Hhk3xTg6r").into_vec().unwrap())),
							base_url: Some("https://node08.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5HGLG8z2jm5KnHeWh2Du8tLLkEVmJ2B6sEnqVsxC2FYjxWRP").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJrEjhUrwArbp7dsHx1QQzf2nSyQQR7B4kNUM4jhhMWPU").into_vec().unwrap())),
							base_url: Some("https://node09.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5HBwJB8DDdXFA8bPRinc8nFuAsZPPFrHLtNeNLvw2jefFN4g").unwrap(), // Never registered
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWHHwE7UFWALqBGHCbt39aacL7ZbBav6sqWXP7jCUuDU1S").into_vec().unwrap())),
							base_url: Some("https://node10.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5EnvH6Lq6LCNZdwS6xbpndEh1YWB51rsDD2pwGJvt13Ta68B").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWGSp3RmUM9JBSKizT4fZLuivTTH82jS3VQDQcFWhM6Pug").into_vec().unwrap())),
							base_url: Some("https://node11.logion.network".as_bytes().to_vec()),
						})
					),
					(
						AccountId::from_str("5Hox4L7Ek1CrXwbYzH8v64WvXkp6rQRkgxuhqE3i2c3farQ9").unwrap(),
						LegalOfficerData::Host(HostData {
							node_id: Some(OpaquePeerId(bs58::decode("12D3KooWHyeYMPuCt69eREeM6AQoFdZVnex5ii5uMHEgfzck1mqU").into_vec().unwrap())),
							base_url: Some("https://node12.logion.network".as_bytes().to_vec()),
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
			relay_chain: "polkadot".into(),
			para_id: main_para_id(),
		},
	)
}

fn main_para_id() -> u32 {
	3341
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
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
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
	endowed_accounts: Vec<(AccountId, Balance)>,
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
			balances: endowed_accounts.iter().cloned().map(|k| (k.0, k.1)).collect(),
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
