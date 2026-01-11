import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { 
  PublicKey, 
  SystemProgram
} from "@solana/web3.js"
import { expect } from "chai";
import {
  createMint,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

import * as spl from "@solana/spl-token"
import { CollateralVault } from "../target/types/collateral_vault";

describe("contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);


  const program = anchor.workspace.collateralVault as Program<CollateralVault>;
  const user = provider.wallet;

  const tokenProgram = spl.TOKEN_PROGRAM_ID;

  it("Is initialized!", async () => {

    const [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    const info = await provider.connection.getAccountInfo(vaultPda);
    console.log("vault data len:", info?.data.length);
    
    const mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      6
    );

    const vaultTokenAccount = await getAssociatedTokenAddress(
      mint,
      vaultPda,
      true
    )
    
    await program.methods
      .initializeVault(vaultBump)
      .accounts({
        user: user.publicKey,
        mint: mint,
        vault: vaultPda,
        vaultTokenAccount: vaultTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const vaultAccount = await program.account.collateralVault.fetch(vaultPda);

    expect(vaultAccount.owner.toBase58()).to.equal(
      user.publicKey.toBase58()
    );
    expect(vaultAccount.tokenAccount.toBase58()).to.equal(
      vaultTokenAccount.toBase58()
    );
    expect(vaultAccount.totalBalance.toNumber()).to.equal(0);
    expect(vaultAccount.lockedBalance.toNumber()).to.equal(0);
    expect(vaultAccount.availableBalance.toNumber()).to.equal(0);
    expect(vaultAccount.totalDeposited.toNumber()).to.equal(0);
    expect(vaultAccount.totalWithdrawn.toNumber()).to.equal(0);

  });
  
});