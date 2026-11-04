use set_core::{PriceEntry, Tx};
use set_ledger::{PriceLedger, TxLedger};
use tempfile::tempdir;

#[test]
fn dual_ledger_roundtrip() {
    let dir = tempdir().unwrap();
    let mut tx = TxLedger::new(dir.path(), "tx");
    let id = tx.append_tx(Tx::Pump { wallet: vec![1], token: 1, amount: 2.0 }).unwrap();
    assert!(tx.contains_tx(id));
    let mut price = PriceLedger::new(dir.path(), "price");
    price.append(PriceEntry { token: 1, location: 2, price: 2, tx_backing: id }).unwrap();
    let reloaded = PriceLedger::new(dir.path(), "price");
    assert_eq!(reloaded.entries.len(), 1);
}
