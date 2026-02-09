export default function VaultCard() {
  return (
    <div className="rounded-xl border p-4">
      <h3 className="font-semibold">USDC Vault</h3>
      <p className="text-sm opacity-70">
        Automated yield on Solana
      </p>
      <button className="mt-3 w-full bg-black text-white py-2 rounded">
        Deposit
      </button>
    </div>
  );
}
