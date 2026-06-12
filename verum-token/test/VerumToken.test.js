const { expect } = require("chai");
const { ethers } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-toolbox/network-helpers");

const MAX_SUPPLY = ethers.parseEther("21000000");
const FEE_BP = 10n; // %0.1
const BP_DENOM = 10000n;

describe("VerumToken", function () {
  // Ortak kurulum: owner hem deployer hem initialHolder, ayrı treasury,
  // alice/bob ücrete tabi normal kullanıcılar.
  async function deployFixture() {
    const [owner, treasury, alice, bob] = await ethers.getSigners();
    const VerumToken = await ethers.getContractFactory("VerumToken");
    const token = await VerumToken.deploy(owner.address, treasury.address);

    // alice'e ücretsiz başlangıç bakiyesi ver (owner muaf olduğu için kesintisiz)
    await token.transfer(alice.address, ethers.parseEther("100000"));

    return { token, owner, treasury, alice, bob };
  }

  describe("Dağıtım (deployment)", function () {
    it("isim ve sembol doğru", async function () {
      const { token } = await loadFixture(deployFixture);
      expect(await token.name()).to.equal("VERUM");
      expect(await token.symbol()).to.equal("VRM");
    });

    it("21M VRM'nin tamamı initialHolder'a basılır", async function () {
      const { token, owner, alice } = await loadFixture(deployFixture);
      const aliceBalance = await token.balanceOf(alice.address);
      const ownerBalance = await token.balanceOf(owner.address);
      expect(ownerBalance + aliceBalance).to.equal(MAX_SUPPLY);
      expect(await token.totalSupply()).to.equal(MAX_SUPPLY);
    });

    it("başlangıç ücreti %0.1 ve treasury ayarlı", async function () {
      const { token, treasury } = await loadFixture(deployFixture);
      expect(await token.feeBasisPoints()).to.equal(FEE_BP);
      expect(await token.treasury()).to.equal(treasury.address);
    });

    it("initialHolder ve treasury ücretten muaf", async function () {
      const { token, owner, treasury } = await loadFixture(deployFixture);
      expect(await token.isFeeExempt(owner.address)).to.be.true;
      expect(await token.isFeeExempt(treasury.address)).to.be.true;
    });

    it("sıfır adresle deploy reddedilir", async function () {
      const [owner] = await ethers.getSigners();
      const VerumToken = await ethers.getContractFactory("VerumToken");
      await expect(
        VerumToken.deploy(ethers.ZeroAddress, owner.address)
      ).to.be.revertedWithCustomError(VerumToken, "ZeroAddress");
      await expect(
        VerumToken.deploy(owner.address, ethers.ZeroAddress)
      ).to.be.revertedWithCustomError(VerumToken, "ZeroAddress");
    });
  });

  describe("Deflasyon mekanizması (whitepaper §9.4)", function () {
    it("transferde %0.1 ücret alınır: yarısı yakılır, yarısı hazineye", async function () {
      const { token, treasury, alice, bob } = await loadFixture(deployFixture);
      const amount = ethers.parseEther("10000");
      const fee = (amount * FEE_BP) / BP_DENOM; // 10 VRM
      const burned = fee / 2n;                  // 5 VRM
      const toTreasury = fee - burned;          // 5 VRM

      const supplyBefore = await token.totalSupply();

      await expect(token.connect(alice).transfer(bob.address, amount))
        .to.emit(token, "FeeTaken")
        .withArgs(alice.address, burned, toTreasury);

      expect(await token.balanceOf(bob.address)).to.equal(amount - fee);
      expect(await token.balanceOf(treasury.address)).to.equal(toTreasury);
      expect(await token.totalSupply()).to.equal(supplyBefore - burned);
      expect(await token.totalBurned()).to.equal(burned);
    });

    it("muaf adres ücret ödemez", async function () {
      const { token, owner, bob } = await loadFixture(deployFixture);
      const amount = ethers.parseEther("1000");
      const supplyBefore = await token.totalSupply();

      await token.connect(owner).transfer(bob.address, amount);

      expect(await token.balanceOf(bob.address)).to.equal(amount);
      expect(await token.totalSupply()).to.equal(supplyBefore);
    });

    it("ücret 0'a çekilirse kesinti olmaz", async function () {
      const { token, alice, bob } = await loadFixture(deployFixture);
      await token.setFeeBasisPoints(0);

      const amount = ethers.parseEther("1000");
      await token.connect(alice).transfer(bob.address, amount);

      expect(await token.balanceOf(bob.address)).to.equal(amount);
    });

    it("toplam arz asla artamaz, sadece azalır", async function () {
      const { token, alice, bob } = await loadFixture(deployFixture);

      for (let i = 0; i < 3; i++) {
        await token.connect(alice).transfer(bob.address, ethers.parseEther("1000"));
        await token.connect(bob).transfer(alice.address, ethers.parseEther("500"));
      }

      expect(await token.totalSupply()).to.be.lessThan(MAX_SUPPLY);
      expect(await token.totalSupply()).to.equal(MAX_SUPPLY - (await token.totalBurned()));
    });
  });

  describe("Yönetim fonksiyonları", function () {
    it("ücret %1 tavanını aşamaz — sahip bile olsa", async function () {
      const { token } = await loadFixture(deployFixture);
      await expect(token.setFeeBasisPoints(101))
        .to.be.revertedWithCustomError(token, "FeeTooHigh")
        .withArgs(101, 100);
      await token.setFeeBasisPoints(100); // tam %1 kabul edilir
      expect(await token.feeBasisPoints()).to.equal(100n);
    });

    it("sahip olmayan ücret/treasury/muafiyet değiştiremez", async function () {
      const { token, alice, bob } = await loadFixture(deployFixture);
      await expect(token.connect(alice).setFeeBasisPoints(50))
        .to.be.revertedWithCustomError(token, "OwnableUnauthorizedAccount");
      await expect(token.connect(alice).setTreasury(bob.address))
        .to.be.revertedWithCustomError(token, "OwnableUnauthorizedAccount");
      await expect(token.connect(alice).setFeeExempt(bob.address, true))
        .to.be.revertedWithCustomError(token, "OwnableUnauthorizedAccount");
    });

    it("treasury güncellenebilir, sıfır adres reddedilir", async function () {
      const { token, bob } = await loadFixture(deployFixture);
      await expect(token.setTreasury(bob.address))
        .to.emit(token, "TreasuryUpdated");
      expect(await token.treasury()).to.equal(bob.address);
      await expect(token.setTreasury(ethers.ZeroAddress))
        .to.be.revertedWithCustomError(token, "ZeroAddress");
    });

    it("muafiyet eklenip kaldırılabilir", async function () {
      const { token, alice, bob } = await loadFixture(deployFixture);

      await token.setFeeExempt(alice.address, true);
      const amount = ethers.parseEther("1000");
      await token.connect(alice).transfer(bob.address, amount);
      expect(await token.balanceOf(bob.address)).to.equal(amount); // kesintisiz

      await token.setFeeExempt(alice.address, false);
      expect(await token.isFeeExempt(alice.address)).to.be.false;
    });

    it("sahiplik devri iki aşamalı (Ownable2Step)", async function () {
      const { token, owner, alice } = await loadFixture(deployFixture);

      await token.transferOwnership(alice.address);
      expect(await token.owner()).to.equal(owner.address); // henüz devrolmadı

      await token.connect(alice).acceptOwnership();
      expect(await token.owner()).to.equal(alice.address);
    });
  });
});
