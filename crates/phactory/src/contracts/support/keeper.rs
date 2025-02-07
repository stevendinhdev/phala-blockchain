use phala_mq::ContractId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use super::{Contract, NativeCompatContract as Compat};
use crate::contracts::{
    pink::Pink,
    data_plaza::DataPlaza,
    balances::Balances,
    assets::Assets,
    web3analytics::Web3Analytics,
    btc_lottery::BtcLottery,
    geolocation::Geolocation,
};

type ContractMap = BTreeMap<ContractId, AnyContract>;

#[derive(Serialize, Deserialize)]
pub enum AnyContract {
    Pink(Compat<Pink>),
    DataPlaza(Compat<DataPlaza>),
    Balances(Compat<Balances>),
    Assets(Compat<Assets>),
    Web3Analytics(Compat<Web3Analytics>),
    BtcLottery(Compat<BtcLottery>),
    Geolocation(Compat<Geolocation>),
}

impl Deref for AnyContract {
    type Target = dyn Contract;

    fn deref(&self) -> &Self::Target {
        match self {
            AnyContract::Pink(c) => c,
            AnyContract::DataPlaza(c) => c,
            AnyContract::Balances(c) => c,
            AnyContract::Assets(c) => c,
            AnyContract::Web3Analytics(c) => c,
            AnyContract::BtcLottery(c) => c,
            AnyContract::Geolocation(c) => c,
        }
    }
}

impl DerefMut for AnyContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AnyContract::Pink(c) => c,
            AnyContract::DataPlaza(c) => c,
            AnyContract::Balances(c) => c,
            AnyContract::Assets(c) => c,
            AnyContract::Web3Analytics(c) => c,
            AnyContract::BtcLottery(c) => c,
            AnyContract::Geolocation(c) => c,
        }
    }
}

impl From<Compat<Pink>> for AnyContract {
    fn from(c: Compat<Pink>) -> Self {
        AnyContract::Pink(c)
    }
}

impl From<Compat<DataPlaza>> for AnyContract {
    fn from(c: Compat<DataPlaza>) -> Self {
        AnyContract::DataPlaza(c)
    }
}

impl From<Compat<Balances>> for AnyContract {
    fn from(c: Compat<Balances>) -> Self {
        AnyContract::Balances(c)
    }
}

impl From<Compat<Assets>> for AnyContract {
    fn from(c: Compat<Assets>) -> Self {
        AnyContract::Assets(c)
    }
}

impl From<Compat<Web3Analytics>> for AnyContract {
    fn from(c: Compat<Web3Analytics>) -> Self {
        AnyContract::Web3Analytics(c)
    }
}

impl From<Compat<BtcLottery>> for AnyContract {
    fn from(c: Compat<BtcLottery>) -> Self {
        AnyContract::BtcLottery(c)
    }
}

impl From<Compat<Geolocation>> for AnyContract {
    fn from(c: Compat<Geolocation>) -> Self {
        AnyContract::Geolocation(c)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct ContractsKeeper(ContractMap);

impl ContractsKeeper {
    pub fn insert(&mut self, contract: impl Into<AnyContract>) {
        let contract = contract.into();
        self.0.insert(contract.id(), contract);
    }

    pub fn keys(&self) -> impl Iterator<Item = &ContractId> {
        self.0.keys()
    }

    pub fn get_mut(&mut self, id: &ContractId) -> Option<&mut AnyContract> {
        self.0.get_mut(id)
    }

    #[cfg(test)]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut AnyContract> {
        self.0.values_mut()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
