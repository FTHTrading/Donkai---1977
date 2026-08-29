// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

import "../src/DonkaiMemoryLedger.sol";
import "../src/DonkaiHumanPass.sol";
import "../src/DonkaiBlindCorroboration.sol";
import "../src/DonkaiEvidenceRegistry.sol";
import "../src/DonkaiReviewRegistry.sol";

contract DonkaiProtocolTest {
    DonkaiMemoryLedger public ledger;
    DonkaiHumanPass public pass;
    DonkaiBlindCorroboration public blind;
    DonkaiEvidenceRegistry public evidence;
    DonkaiReviewRegistry public review;

    address alice = address(0x1111);
    address bob = address(0x2222);

    function setUp() public {
        ledger = new DonkaiMemoryLedger();
        pass = new DonkaiHumanPass();
        blind = new DonkaiBlindCorroboration();
        evidence = new DonkaiEvidenceRegistry();
        review = new DonkaiReviewRegistry();
    }

    function test_commit_and_get_record() public {
        bytes32 stmtRoot = keccak256("statement_test");
        bytes32 ctxRoot = keccak256("context_test");
        bytes32 consentRoot = keccak256("consent_test");
        bytes32 evRoot = keccak256("evidence_test");
        bytes32 parentRoot = bytes32(0);
        bytes32 methodology = keccak256("v0.1.0");

        uint256 id = ledger.commitRecord(
            stmtRoot,
            ctxRoot,
            consentRoot,
            evRoot,
            parentRoot,
            methodology,
            DonkaiMemoryLedger.VisibilityMode.Public
        );

        assert(id == 1);
        DonkaiMemoryLedger.MemoryCommitment memory rec = ledger.getRecord(1);
        assert(rec.statementRoot == stmtRoot);
        assert(rec.status == DonkaiMemoryLedger.RecordStatus.Active);
    }

    function test_human_pass_non_transferable() public {
        bytes32 idHash = keccak256("alice_identity");
        uint256 passId = pass.issuePass(alice, idHash);
        assert(passId == 1);
        assert(pass.isHumanVerified(alice) == true);
        assert(pass.isHumanVerified(bob) == false);
    }

    function test_blind_corroboration_commit_reveal() public {
        bytes32 salt = keccak256("secret_salt_1977");
        bytes memory narrative = bytes("I remember the Space Invaders cabinet on Main Street.");

        // Compute sealed recall root
        bytes32 sealedRoot = sha256(
            abi.encodePacked("DONKAI:LPS1:BLIND_CORROBORATION:v1:", salt, narrative)
        );

        bytes32 discoveryHash = keccak256("austin_arcade_1978");

        // Phase 1: Commit sealed root
        uint256 corrobId = blind.commitSealedRecall(1, sealedRoot, discoveryHash);
        assert(corrobId == 1);

        // Phase 2: Reveal
        blind.revealRecall(corrobId, narrative, salt);
        assert(blind.revealedCount(1) == 1);
    }
}
