// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title DonkaiTranslationRegistry
 * @author DONK AI Core Protocol Engineering
 * @notice Maintains signed derivative translation roots anchored to canonical original statements.
 * Enforces original-language primacy and translator accountability.
 */
contract DonkaiTranslationRegistry {
    struct TranslationRef {
        uint256 memoryId;
        bytes32 originalStatementRoot;
        bytes32 translationRoot;
        string targetLanguageBcp47;
        address translator;
        uint64 attestedAt;
        uint32 humanAttestationCount;
    }

    uint256 private _nextTranslationId = 1;
    mapping(uint256 => TranslationRef) public translations;
    mapping(uint256 => uint256[]) public memoryTranslations;

    event TranslationRegistered(
        uint256 indexed translationId,
        uint256 indexed memoryId,
        bytes32 indexed translationRoot,
        string targetLanguage,
        address translator,
        uint64 attestedAt
    );

    event TranslationAttested(uint256 indexed translationId, address attester);

    function registerTranslation(
        uint256 memoryId,
        bytes32 originalStatementRoot,
        bytes32 translationRoot,
        string calldata targetLanguage
    ) external returns (uint256 translationId) {
        translationId = _nextTranslationId++;
        translations[translationId] = TranslationRef({
            memoryId: memoryId,
            originalStatementRoot: originalStatementRoot,
            translationRoot: translationRoot,
            targetLanguageBcp47: targetLanguage,
            translator: msg.sender,
            attestedAt: uint64(block.timestamp),
            humanAttestationCount: 1
        });

        memoryTranslations[memoryId].push(translationId);

        emit TranslationRegistered(
            translationId,
            memoryId,
            translationRoot,
            targetLanguage,
            msg.sender,
            uint64(block.timestamp)
        );
    }

    function attestTranslation(uint256 translationId) external {
        TranslationRef storage t = translations[translationId];
        require(t.attestedAt != 0, "Translation not found");
        t.humanAttestationCount++;
        emit TranslationAttested(translationId, msg.sender);
    }

    function getTranslationIds(uint256 memoryId) external view returns (uint256[] memory) {
        return memoryTranslations[memoryId];
    }
}
