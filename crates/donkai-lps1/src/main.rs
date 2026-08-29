use donkai_lps1::{
    canonicalize, canonicalize_json_str, hash_leaf, Commitment, MemoryRecord, MerkleProof,
    MerkleTree, RemembranceStatement, Validator,
};
use std::env;
use std::fs;
use std::process::exit;

fn print_help() {
    println!("DONK AI — LPS-1 Living Provenance Standard CLI");
    println!("Usage:");
    println!("  donkai-lps1 validate <file.json>                Validate a remembrance or memory record");
    println!("  donkai-lps1 commit <file.json>                  Compute canonical bytes and LPS-1 commitment root");
    println!("  donkai-lps1 bundle <evidence-manifest.json>     Compute Merkle root for an evidence bundle");
    println!("  donkai-lps1 prove <bundle.json> --leaf <hash>   Generate inclusion proof for a leaf");
    println!("  donkai-lps1 verify-proof <proof.json>           Verify a Merkle inclusion proof");
    println!("  donkai-lps1 verify <commitment.json> --input <file.json>");
    println!("                                                  Verify commitment against input JSON");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "validate" => {
            if args.len() < 3 {
                eprintln!("Error: Missing input file path");
                exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).expect("Failed to read file");
            
            if let Ok(record) = serde_json::from_str::<MemoryRecord>(&content) {
                let report = Validator::validate_memory_record(&record).expect("Validation error");
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else if let Ok(stmt) = serde_json::from_str::<RemembranceStatement>(&content) {
                let report = Validator::validate_remembrance(&stmt).expect("Validation error");
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                eprintln!("Error: Unrecognized record format");
                exit(1);
            }
        }
        "commit" => {
            if args.len() < 3 {
                eprintln!("Error: Missing input file path");
                exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).expect("Failed to read file");
            let canon = canonicalize_json_str(&content).expect("Failed to canonicalize JSON");
            let commitment = Commitment::from_canonical("remembrance", canon.as_bytes())
                .expect("Failed to create commitment");
            println!("{}", serde_json::to_string_pretty(&commitment).unwrap());
        }
        "bundle" => {
            if args.len() < 3 {
                eprintln!("Error: Missing manifest path");
                exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).expect("Failed to read manifest");
            let manifest: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON");
            let canon = canonicalize(&manifest).expect("Failed to canonicalize manifest");
            let root = hash_leaf("evidence_manifest", canon.as_bytes());
            println!("{{\"bundleRoot\": \"0x{}\", \"bytes\": {}}}", hex::encode(root), canon.len());
        }
        "prove" => {
            if args.len() < 5 || args[3] != "--leaf" {
                eprintln!("Usage: donkai-lps1 prove <bundle.json> --leaf <hash>");
                exit(1);
            }
            let path = &args[2];
            let leaf_hex = args[4].strip_prefix("0x").unwrap_or(&args[4]);
            let target_leaf: [u8; 32] = hex::decode(leaf_hex)
                .expect("Invalid leaf hex")
                .try_into()
                .expect("Expected 32-byte leaf");

            let content = fs::read_to_string(path).expect("Failed to read file");
            let leaves_val: Vec<String> = serde_json::from_str(&content).expect("Expected array of hex hashes");
            let leaves: Vec<[u8; 32]> = leaves_val
                .iter()
                .map(|s| {
                    let clean = s.strip_prefix("0x").unwrap_or(s);
                    let mut b = [0u8; 32];
                    b.copy_from_slice(&hex::decode(clean).unwrap());
                    b
                })
                .collect();

            let tree = MerkleTree::build(leaves.clone()).expect("Failed to build tree");
            let idx = leaves.iter().position(|l| l == &target_leaf).expect("Leaf not found in tree");
            let proof = tree.generate_proof(idx).expect("Failed to generate proof");
            println!("{}", serde_json::to_string_pretty(&proof).unwrap());
        }
        "verify-proof" => {
            if args.len() < 3 {
                eprintln!("Error: Missing proof file path");
                exit(1);
            }
            let path = &args[2];
            let content = fs::read_to_string(path).expect("Failed to read proof file");
            let proof: MerkleProof = serde_json::from_str(&content).expect("Invalid proof JSON");
            match proof.verify(None) {
                Ok(true) => println!("{{\"verified\": true, \"root\": \"0x{}\"}}", hex::encode(proof.root)),
                _ => {
                    println!("{{\"verified\": false}}");
                    exit(1);
                }
            }
        }
        "verify" => {
            if args.len() < 5 || args[3] != "--input" {
                eprintln!("Usage: donkai-lps1 verify <commitment.json> --input <file.json>");
                exit(1);
            }
            let commit_path = &args[2];
            let input_path = &args[4];

            let commit_content = fs::read_to_string(commit_path).expect("Failed to read commitment file");
            let commitment: Commitment = serde_json::from_str(&commit_content).expect("Invalid commitment JSON");

            let input_content = fs::read_to_string(input_path).expect("Failed to read input file");
            let canon = canonicalize_json_str(&input_content).expect("Failed to canonicalize input");
            let computed_leaf = hash_leaf("remembrance", canon.as_bytes());

            if computed_leaf == commitment.root {
                println!("{{\"valid\": true, \"matchingRoot\": \"0x{}\"}}", hex::encode(computed_leaf));
            } else {
                println!(
                    "{{\"valid\": false, \"expected\": \"0x{}\", \"computed\": \"0x{}\"}}",
                    hex::encode(commitment.root),
                    hex::encode(computed_leaf)
                );
                exit(1);
            }
        }
        _ => {
            print_help();
        }
    }
}
