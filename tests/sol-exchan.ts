import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolExchan } from "../target/types/sol_exchan";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
const { PublicKey, Keypair, SystemProgram } = anchor.web3;
const metadataProgramId = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"); // Metaplex metadata program ID

let tokenMint: anchor.web3.PublicKey;
describe("sol-exchan", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolExchan as Program<SolExchan>;

  it("Is initialized!", async () => {

    // await program.methods.initialize(200, new anchor.BN(10)).accounts(
    //     {feesWallet: program.provider.publicKey, signer: program.provider.publicKey}
    // ).rpc({skipPreflight: true});
});


it("Create Token", async () => {
  let tokenMint_ = anchor.web3.Keypair.generate();
  tokenMint = tokenMint_.publicKey;
  const metadataAccount = await findMetadataAccount(tokenMint_.publicKey);

  let tx=await program.methods.createToken("test", "TEST", "https://metadata.drift.foundation/drift.json").accounts({
      creator: program.provider.publicKey,
      mint: tokenMint,
      bondingCurve: findBondingCurve(tokenMint),
      tokenCustody: findAssociatedTokenAddress(findBondingCurve(tokenMint), tokenMint),
      metadataAccount: metadataAccount,// provide the correct metadata account here,
      tokenMetadataProgram: metadataProgramId,// provide the correct token metadata program here,
      tokenProgram: TOKEN_PROGRAM_ID,
      SystemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,

  }).signers([tokenMint_]).rpc();

  console.log(tx);

});
});

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: anchor.web3.PublicKey =
  new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

function findAssociatedTokenAddress(
  walletAddress: anchor.web3.PublicKey,
  tokenMintAddress: anchor.web3.PublicKey
): anchor.web3.PublicKey {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      walletAddress.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      tokenMintAddress.toBuffer(),
    ],
    SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
  )[0];
}

function findBondingCurve(tokenMintAddress: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bonding_curve"), tokenMintAddress.toBuffer()],
    new anchor.web3.PublicKey("8gm6UpJoDAawsGYrtGXhatokbFQDVZeBkUP1j8egxKTp")
  )[0];
}


async function findMetadataAccount(mint) {
  return (
    await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        metadataProgramId.toBuffer(),
        mint.toBuffer(),
      ],
      metadataProgramId
    )
  )[0];
}

