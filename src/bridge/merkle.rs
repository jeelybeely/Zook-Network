use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: MerkleNode,
    pub leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    pub fn new(transaction_hashes: Vec<Vec<u8>>) -> Self {
        assert!(!transaction_hashes.is_empty(), "Transaction hashes cannot be empty");

        let leaves = transaction_hashes
            .into_iter()
            .map(|hash| MerkleNode { hash })
            .collect::<Vec<_>>();

        println!("Leaves: {:?}", leaves); // Debugging: Print leaves

        let root = Self::build_tree(&leaves);

        MerkleTree { root, leaves }
    }

    fn build_tree(nodes: &[MerkleNode]) -> MerkleNode {
        if nodes.len() == 1 {
            return nodes[0].clone();
        }

        println!("Building tree layer: {:?}", nodes); // Debugging: Print current layer

        let mut parent_nodes = vec![];

        for chunk in nodes.chunks(2) {
            let combined_hash = match chunk {
                [left, right] => {
                    println!("Combining: {:?} + {:?}", left.hash, right.hash); // Debugging
                    let mut hasher = Sha256::new();
                    let (smaller, larger) = if left.hash < right.hash {
                        (&left.hash, &right.hash)
                    } else {
                        (&right.hash, &left.hash)
                    };
                    hasher.update(smaller);
                    hasher.update(larger);
                    hasher.finalize().to_vec()
                }
                [left] => {
                    println!("Single node duplicated: {:?}", left.hash); // Debugging
                    let mut hasher = Sha256::new();
                    hasher.update(&left.hash);
                    hasher.update(&left.hash);
                    hasher.finalize().to_vec()
                }
                _ => unreachable!(),
            };

            parent_nodes.push(MerkleNode { hash: combined_hash });
        }

        Self::build_tree(&parent_nodes)
    }

    pub fn get_root(&self) -> Vec<u8> {
        self.root.hash.clone()
    }

    pub fn get_root_as_string(&self) -> Result<String, String> {
        String::from_utf8(self.root.hash.clone())
            .map_err(|_| "Failed to convert root hash to string".to_string())
    }

    pub fn get_proof(&self, transaction_hash: &[u8]) -> Vec<Vec<u8>> {
        let mut proof = vec![];
        let mut index = self.leaves.iter().position(|node| node.hash == transaction_hash);

        if index.is_none() {
            return proof; // Transaction not found
        }

        let mut current_layer = self.leaves.clone();

        while current_layer.len() > 1 {
            println!("Current Layer: {:?}", current_layer); // Debugging: Print current layer

            let mut next_layer = vec![];

            for (i, chunk) in current_layer.chunks(2).enumerate() {
                match chunk {
                    [left, right] => {
                        if i == index.unwrap() / 2 {
                            if index.unwrap() % 2 == 0 {
                                proof.push(right.hash.clone());
                                println!("Proof node added: {:?}", right.hash); // Debugging
                            } else {
                                proof.push(left.hash.clone());
                                println!("Proof node added: {:?}", left.hash); // Debugging
                            }
                        }

                        let mut hasher = Sha256::new();
                        let (smaller, larger) = if left.hash < right.hash {
                            (&left.hash, &right.hash)
                        } else {
                            (&right.hash, &left.hash)
                        };
                        hasher.update(smaller);
                        hasher.update(larger);
                        next_layer.push(MerkleNode {
                            hash: hasher.finalize().to_vec(),
                        });
                    }
                    [left] => {
                        next_layer.push(left.clone());
                    }
                    _ => unreachable!(),
                }
            }

            current_layer = next_layer;
            index = index.map(|i| i / 2);
        }

        proof
    }

    pub fn verify_proof(proof: Vec<Vec<u8>>, root_hash: Vec<u8>, transaction_hash: Vec<u8>) -> bool {
        let mut computed_hash = transaction_hash;

        for (i, sibling_hash) in proof.iter().enumerate() {
            let mut hasher = Sha256::new();

            // Ensure consistent ordering of sibling hashes
            let (smaller, larger) = if computed_hash < *sibling_hash {
                (&computed_hash, sibling_hash)
            } else {
                (sibling_hash, &computed_hash)
            };

            println!("Hash Order: {:?} + {:?}", smaller, larger); // Debugging
            hasher.update(smaller);
            hasher.update(larger);

            computed_hash = hasher.finalize().to_vec();
            println!(
                "Step {}: Computed Hash: {:?}, Sibling Hash: {:?}",
                i + 1,
                computed_hash,
                sibling_hash
            );
        }

        println!(
            "Final Computed Hash: {:?}, Expected Root Hash: {:?}",
            computed_hash, root_hash
        );

        computed_hash == root_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree() {
        let transactions = vec![
            b"tx1".to_vec(),
            b"tx2".to_vec(),
            b"tx3".to_vec(),
            b"tx4".to_vec(),
        ];

        let merkle_tree = MerkleTree::new(transactions.clone());
        let root = merkle_tree.get_root();

        for tx in &transactions {
            let proof = merkle_tree.get_proof(tx);
            println!("Transaction: {:?}, Proof: {:?}", tx, proof); // Debugging logs
            assert!(
                MerkleTree::verify_proof(proof.clone(), root.clone(), tx.clone()),
                "Failed to verify proof for transaction {:?}",
                tx
            );
        }
    }

    #[test]
    fn test_invalid_transaction() {
        let transactions = vec![
            b"tx1".to_vec(),
            b"tx2".to_vec(),
            b"tx3".to_vec(),
            b"tx4".to_vec(),
        ];

        let merkle_tree = MerkleTree::new(transactions);
        let root = merkle_tree.get_root();
        let fake_transaction = b"fake_tx".to_vec();

        let proof = merkle_tree.get_proof(&fake_transaction);
        println!("Fake Transaction Proof: {:?}", proof); // Debugging logs
        assert!(!MerkleTree::verify_proof(proof, root, fake_transaction));
    }
}
