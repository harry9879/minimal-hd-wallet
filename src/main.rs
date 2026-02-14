use miniscript::{
    bitcoin::secp256k1::Secp256k1, bitcoin::ScriptBuf, Descriptor, DescriptorPublicKey,
};
use std::collections::BTreeMap;
use std::str::FromStr;

pub struct SimpleWallet {
    descriptor: Descriptor<DescriptorPublicKey>,
    last_revealed: Option<u32>,
    lookahead: u32,
    spks: BTreeMap<u32, ScriptBuf>,
}

impl SimpleWallet {
    pub fn new(desc_str: &str, lookahead: u32) -> Self {
        let descriptor =
            Descriptor::<DescriptorPublicKey>::from_str(desc_str).expect("invalid descriptor");

        let mut wallet = Self {
            descriptor,
            last_revealed: None,
            lookahead,
            spks: BTreeMap::new(),
        };

        wallet.replenish();

        wallet
    }

    pub fn derive_script(&self, index: u32) -> ScriptBuf {
        let secp = Secp256k1::new();

        self.descriptor
            .derived_descriptor(&secp, index)
            .expect("derivation failed")
            .script_pubkey()
    }

    pub fn reveal_next(&mut self) -> (u32, ScriptBuf) {
        let next_index = self.last_revealed.map(|i| i + 1).unwrap_or(0);

        self.last_revealed = Some(next_index);

        self.replenish();

        let script = self.spks.get(&next_index).unwrap().clone();

        (next_index, script)
    }

    fn replenish(&mut self) {
        let secp = Secp256k1::new();

        let next_reveal = self.last_revealed.map(|i| i + 1).unwrap_or(0);
        let stop = next_reveal + self.lookahead;

        let next_index = self.spks.keys().last().map(|i| i + 1).unwrap_or(0);

        for i in next_index..stop {
            let script = self
                .descriptor
                .derived_descriptor(&secp, i)
                .expect("derivation failed")
                .script_pubkey();

            self.spks.insert(i, script);
        }
    }
}

fn main() {
    let desc_str = "wpkh([25fa2d23/84h/1h/0h]tpubDDKKME7KZWqhDaK9VXA4sBxNQvoRbRsGnxxmvcQv5NwoZyjkjNBjjU9PEh7taW5reRD6jnX6qj5VXp3YzyeVrcyQB9vBUfjLkBhoZjQMqND/1/*)";

    let mut wallet = SimpleWallet::new(desc_str, 5);

    let (i1, _) = wallet.reveal_next();
    let (i2, _) = wallet.reveal_next();

    println!("Revealed: {}", i1);
    println!("Revealed: {}", i2);

    println!("Stored scripts: {}", wallet.spks.len());
}
