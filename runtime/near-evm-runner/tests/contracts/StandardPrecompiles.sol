// SPDX-License-Identifier: Unlicense
pragma solidity ^0.7.0;

contract StandardPrecompiles {
  constructor() payable {}

  function test_ecrecover() public pure returns(bool) {
    bytes32 hash = hex"1111111111111111111111111111111111111111111111111111111111111111";
    bytes memory sig = hex"b9f0bb08640d3c1c00761cdd0121209268f6fd3816bc98b9e6f3cc77bf82b69812ac7a61788a0fdc0e19180f14c945a8e1088a27d92a74dce81c0981fb6447441b";
    address signer = 0x1563915e194D8CfBA1943570603F7606A3115508;
    return ecverify(hash, sig, signer);
  }

  function ecverify(bytes32 hash, bytes memory sig, address signer) private pure returns (bool) {
    bool ret;
    address addr;
    (ret, addr) = ecrecovery(hash, sig);
    return ret && addr == signer;
  }

  function ecrecovery(bytes32 hash, bytes memory sig) private pure returns (bool, address) {
    bytes32 r;
    bytes32 s;
    uint8 v;

    if (sig.length != 65)
      return (false, address(0));

    assembly {
      r := mload(add(sig, 32))
      s := mload(add(sig, 64))
      v := byte(0, mload(add(sig, 96)))
    }

    address addr = ecrecover(hash, v, r, s);
    return (true, addr);
  }
}
