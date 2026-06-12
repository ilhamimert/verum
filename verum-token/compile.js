// VerumToken.sol derleme kontrolü — solc + OpenZeppelin import çözümü
const fs = require("fs");
const path = require("path");
const solc = require("solc");

const CONTRACT_PATH = path.join(__dirname, "contracts", "VerumToken.sol");

function findImports(importPath) {
  const candidates = [
    path.join(__dirname, "node_modules", importPath),
    path.join(__dirname, "contracts", importPath),
  ];
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) {
      return { contents: fs.readFileSync(candidate, "utf8") };
    }
  }
  return { error: `Import bulunamadı: ${importPath}` };
}

const input = {
  language: "Solidity",
  sources: {
    "VerumToken.sol": { content: fs.readFileSync(CONTRACT_PATH, "utf8") },
  },
  settings: {
    optimizer: { enabled: true, runs: 200 },
    outputSelection: { "*": { "*": ["abi", "evm.bytecode.object"] } },
  },
};

const output = JSON.parse(
  solc.compile(JSON.stringify(input), { import: findImports })
);

const errors = (output.errors || []).filter((e) => e.severity === "error");
const warnings = (output.errors || []).filter((e) => e.severity === "warning");

warnings.forEach((w) => console.warn("UYARI:", w.formattedMessage));

if (errors.length > 0) {
  errors.forEach((e) => console.error("HATA:", e.formattedMessage));
  process.exit(1);
}

const contract = output.contracts["VerumToken.sol"]["VerumToken"];
const bytecodeSize = contract.evm.bytecode.object.length / 2;

console.log("✅ VerumToken.sol başarıyla derlendi");
console.log(`   Bytecode boyutu: ${bytecodeSize} byte (limit: 24576)`);
console.log(`   ABI fonksiyon sayısı: ${contract.abi.length}`);

fs.writeFileSync(
  path.join(__dirname, "VerumToken.abi.json"),
  JSON.stringify(contract.abi, null, 2)
);
console.log("   ABI yazıldı: VerumToken.abi.json");
