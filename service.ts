import * as anchor from "@coral-xyz/anchor";

export async function rebalanceVault(program: anchor.Program) {
  console.log("Rebalancing Kamilend vault on Solana...");
  await program.methods.rebalance().rpc();
}
