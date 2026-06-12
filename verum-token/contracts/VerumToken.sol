// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/// @title VERUM (VRM) — ERC-20 Token
/// @notice Whitepaper v1.0 tokenomics'inin birebir uygulaması:
///         - Toplam arz: 21.000.000 VRM (sabit, sonradan mint YOK)
///         - Deflasyon: her transferde alınan ücretin %50'si yakılır,
///           %50'si hazineye (vakıf) gider
///         - Ücret oranı en fazla %1 olabilir (sahip bile aşamaz)
/// @dev OpenZeppelin v5 üzerine inşa edilmiştir. Takım kilidi (4 yıl) için
///      ayrıca OpenZeppelin VestingWallet kullanın — README'ye bakın.
contract VerumToken is ERC20, ERC20Permit, Ownable2Step {
    /// @notice Sabit maksimum arz — constructor'da bir kez basılır.
    uint256 public constant MAX_SUPPLY = 21_000_000 * 1e18;

    /// @notice Ücret üst sınırı: 100 baz puan = %1. Değiştirilemez.
    uint256 public constant MAX_FEE_BASIS_POINTS = 100;

    uint256 private constant BASIS_POINTS_DENOMINATOR = 10_000;

    /// @notice Transfer ücreti (baz puan). 10 = %0.1
    uint256 public feeBasisPoints;

    /// @notice Yakılmayan ücret payının gittiği vakıf/hazine adresi.
    address public treasury;

    /// @notice Ücretten muaf adresler (ör. likidite havuzu, vesting sözleşmesi).
    mapping(address => bool) public isFeeExempt;

    /// @notice Bugüne kadar yakılan toplam VRM.
    uint256 public totalBurned;

    event FeeBasisPointsUpdated(uint256 oldFee, uint256 newFee);
    event TreasuryUpdated(address oldTreasury, address newTreasury);
    event FeeExemptionUpdated(address indexed account, bool exempt);
    event FeeTaken(address indexed from, uint256 burned, uint256 toTreasury);

    error ZeroAddress();
    error FeeTooHigh(uint256 requested, uint256 maxAllowed);

    /// @param initialHolder Tüm arzın basılacağı adres (dağıtımı README'deki
    ///        tokenomics tablosuna göre yapın).
    /// @param treasury_ Ücretin yakılmayan yarısının gideceği vakıf adresi.
    constructor(address initialHolder, address treasury_)
        ERC20("VERUM", "VRM")
        ERC20Permit("VERUM")
        Ownable(msg.sender)
    {
        if (initialHolder == address(0) || treasury_ == address(0)) {
            revert ZeroAddress();
        }

        treasury = treasury_;
        feeBasisPoints = 10; // %0.1 başlangıç ücreti

        // Dağıtım ve vesting transferleri ücretsiz olsun:
        isFeeExempt[initialHolder] = true;
        isFeeExempt[treasury_] = true;

        _mint(initialHolder, MAX_SUPPLY);
    }

    // ─────────────────────────── Yönetim ───────────────────────────

    /// @notice Ücret oranını günceller. %1'i (100 bp) asla aşamaz.
    function setFeeBasisPoints(uint256 newFee) external onlyOwner {
        if (newFee > MAX_FEE_BASIS_POINTS) {
            revert FeeTooHigh(newFee, MAX_FEE_BASIS_POINTS);
        }
        emit FeeBasisPointsUpdated(feeBasisPoints, newFee);
        feeBasisPoints = newFee;
    }

    /// @notice Hazine adresini günceller.
    function setTreasury(address newTreasury) external onlyOwner {
        if (newTreasury == address(0)) revert ZeroAddress();
        emit TreasuryUpdated(treasury, newTreasury);
        treasury = newTreasury;
    }

    /// @notice Bir adresi transfer ücretinden muaf tutar / muafiyeti kaldırır.
    /// @dev DEX likidite havuzları ve vesting sözleşmeleri için gereklidir;
    ///      fee-on-transfer bazı protokollerle uyumsuz olabilir.
    function setFeeExempt(address account, bool exempt) external onlyOwner {
        if (account == address(0)) revert ZeroAddress();
        isFeeExempt[account] = exempt;
        emit FeeExemptionUpdated(account, exempt);
    }

    // ─────────────────────── Transfer + Deflasyon ───────────────────────

    /// @dev Her transferde ücret alınır: %50'si yakılır (totalSupply düşer),
    ///      %50'si hazineye gider. Mint/burn ve muaf adresler ücretsizdir.
    function _update(address from, address to, uint256 value)
        internal
        override
    {
        bool feeless = feeBasisPoints == 0
            || from == address(0) // mint
            || to == address(0)   // burn
            || isFeeExempt[from]
            || isFeeExempt[to];

        if (feeless) {
            super._update(from, to, value);
            return;
        }

        uint256 fee = (value * feeBasisPoints) / BASIS_POINTS_DENOMINATOR;
        uint256 burnAmount = fee / 2;
        uint256 treasuryAmount = fee - burnAmount;

        if (burnAmount > 0) {
            super._update(from, address(0), burnAmount);
            totalBurned += burnAmount;
        }
        if (treasuryAmount > 0) {
            super._update(from, treasury, treasuryAmount);
        }
        super._update(from, to, value - fee);

        emit FeeTaken(from, burnAmount, treasuryAmount);
    }
}
