// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract MirrorVerifier {
    struct MirrorRecord {
        address creator;
        bytes32 commitment;
        uint64 blockHeight;
        uint64 timestampMs;
    }

    mapping(bytes32 => MirrorRecord) public mirrors;
    mapping(address => bool) public authorizedSigners;

    event MirrorPosted(address indexed creator, bytes32 indexed commitment, uint64 blockHeight, uint64 timestampMs);

    constructor() {
        authorizedSigners[msg.sender] = true;
    }

    function postMirror(
        address creator,
        bytes32 commitment,
        uint64 blockHeight,
        uint64 timestampMs,
        bytes calldata signature
    ) external {
        require(mirrors[commitment].creator == address(0), "Already mirrored");
        require(authorizedSigners[creator], "Not authorized");

        bytes32 digest = keccak256(abi.encodePacked(creator, commitment, blockHeight, timestampMs));
        address recovered = recoverSigner(digest, signature);
        require(recovered == creator, "Invalid signature");

        mirrors[commitment] = MirrorRecord({
            creator: creator,
            commitment: commitment,
            blockHeight: blockHeight,
            timestampMs: timestampMs
        });

        emit MirrorPosted(creator, commitment, blockHeight, timestampMs);
    }

    function recoverSigner(bytes32 digest, bytes calldata sig) internal pure returns (address) {
        bytes32 ethSigned = ECDSA.toEthSignedMessageHash(digest);
        return ECDSA.recover(ethSigned, sig);
    }

    function authorize(address signer, bool enabled) external {
        require(msg.sender == signer || msg.sender == address(this), "Not permitted");
        authorizedSigners[signer] = enabled;
    }
}

library ECDSA {
    function toEthSignedMessageHash(bytes32 hash) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }

    function recover(bytes32 hash, bytes memory signature) internal pure returns (address) {
        if (signature.length != 65) revert("Bad signature length");
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(signature, 32))
            s := mload(add(signature, 64))
            v := byte(0, mload(add(signature, 96)))
        }
        if (v < 27) v += 27;
        require(v == 27 || v == 28, "Bad v value");
        return ecrecover(hash, v, r, s);
    }
}
