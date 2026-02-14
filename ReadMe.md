we'll build a minimal mental clone of keychainTxoutIndex with 

single descriptor
Reveal logic
Lookahead 
Index txouts
Track Utxos

Holds one descriptor
Tracks:
last_revealed index
derived script pubkeys
indexed utxos
Has lookahead
Can:
reveal next address
scan a transaction
compute balance