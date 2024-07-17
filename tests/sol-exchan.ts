import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolExchan } from "../target/types/sol_exchan";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
const { Connection } = require('@solana/web3.js');

describe("sol-exchan", () => {
  let provider=anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolExchan as Program<SolExchan>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("token transfer", async () => {

    const secretKeyString = "FiucH9JdTUkBRBryDqCYNAWe7yjneYTY3gqr8ErdN9xX6ySLczLP2hnxr5Uu4ZGemgNTnBGQWvQNfkPytQ7YFx8";
  const secretKeyBuffer = bs58.decode(secretKeyString);
  const secretKeyUint8Array = new Uint8Array(secretKeyBuffer);
  const buyerWallet = anchor.web3.Keypair.fromSecretKey(secretKeyUint8Array);

  const connection = new Connection(provider.connection.rpcEndpoint);

  // Ensure the buyer wallet has an associated token account for the specified mint
  const tokenMint = new anchor.web3.PublicKey('HRL4ExfreCMnTAeGo6AXE9qLssYfGUMo3WHmwUJWmyX'); // Replace with your token mint address
  const fromTokenAccount = new anchor.web3.PublicKey('2cKwne8UsGVyABN1DCMdnsY8xoNaGKpcmBB9wJYJhsdb');

  // Get or create associated token account for the buyer
  const buyerTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    buyerWallet,
    tokenMint,
    buyerWallet.publicKey
  );

  try {
    await program.rpc.swapTokens(new anchor.BN(10), {
      accounts: {
        authority: provider.wallet.publicKey,
        tokenAccount: fromTokenAccount,
        buyerWallet: buyerTokenAccount.address,
        tokenProgram:new anchor.web3.PublicKey("11111111111111111111111111111111"),
      },
      signers: [],
    });
  } catch (error) {
    console.error("Error swapping tokens:", error);
    if (error.logs) {
      console.log("Transaction logs:", error.logs);
    }
    throw error;
  }
  });
});
