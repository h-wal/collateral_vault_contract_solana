import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { 
  PublicKey, 
  SystemProgram
} from "@solana/web3.js"
import { expect, use } from "chai";
import {
  createMint,
  getAssociatedTokenAddressSync,
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  mintTo
} from "@solana/spl-token";

import { CollateralVault } from "../target/types/collateral_vault";

describe("Collateral_Vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);


  const program = anchor.workspace.collateralVault as Program<CollateralVault>;
  const user = provider.wallet;

  const [vaultAuthorityPda, vaultAuthorityBump] =
  PublicKey.findProgramAddressSync(
    [Buffer.from("vault_authority")],
    program.programId
  );

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
      6,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const vaultTokenAccount = getAssociatedTokenAddressSync (
      mint,
      vaultPda,
      true,
      TOKEN_2022_PROGRAM_ID
    )
    
    await program.methods
      .initializeVault(vaultBump)
      .accounts({
        user: user.publicKey,
        mint: mint,
        vault: vaultPda,
        vaultTokenAccount: vaultTokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
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
  
  it("deposits", async () => {

    const [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    const vaultAccount = await program.account.collateralVault.fetch(vaultPda);
    const vaultTokenAccount = vaultAccount.tokenAccount;

    const vaultTokenAccountInfo = await provider.connection.getParsedAccountInfo(vaultTokenAccount);

    const mint = new PublicKey(
      //@ts-ignore
      vaultTokenAccountInfo.value.data.parsed.info.mint
    );

    const userTokenAccount = getAssociatedTokenAddressSync(
      mint, 
      user.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    );

    try {
      await createAssociatedTokenAccount(
        provider.connection,
        provider.wallet.payer,
        mint,
        user.publicKey,
        undefined,
        TOKEN_2022_PROGRAM_ID
      )
    } catch (_){
      //can be ignored
    }
    

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mint,
      userTokenAccount,
      user.publicKey,
      1_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    )

    await program.methods
      .deposit( new anchor.BN(500_000))
      .accounts({
        user: user.publicKey,
        vault: vaultPda,
        userTokenAccount,
        vaultTokenAccount,
        mint,
        tokenProgram: TOKEN_2022_PROGRAM_ID
      })
      .rpc();

      const updatedVault = await program.account.collateralVault.fetch(vaultPda);

      console.log(updatedVault)

      expect(updatedVault.totalBalance.toNumber()).to.equal(500_000);
      expect(updatedVault.availableBalance.toNumber()).to.equal(500_000);
      expect(updatedVault.totalDeposited.toNumber()).to.equal(500_000);

  });

  it("inintializes vault authority", async () => {

    await program.methods.initializeVaultAuthority(vaultAuthorityBump).accounts({
      vaultAuthority: vaultAuthorityPda,
      payer: user.publicKey,
      systemProgram: SystemProgram.programId
    }).rpc();

    const authority = await program.account.vaultAuthority.fetch(
      vaultAuthorityPda
    );

    expect(authority.admin.toBase58()).to.equal(
      user.publicKey.toBase58()
    );

    expect(authority.authorizedPrograms.length).to.equal(0);

  });

  it("adds authorized programs", async () => {
    await program.methods.addAuthorizedProgram(program.programId).accounts({
      vaultAuthority: vaultAuthorityPda,
      admin: user.publicKey
    }).rpc();

    const authority = await program.account.vaultAuthority.fetch(
      vaultAuthorityPda
    );

    expect(
      authority.authorizedPrograms[0].toBase58()
    ).to.equal(program.programId.toBase58());
  })

  it("locks collateral", async () => {

    const [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    const vaultAccountBefore = await program.account.collateralVault.fetch(vaultPda);

    const lockAmount = 400_000;

    const [vaultAuthorityPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_authority")],
      program.programId
    );

    await program.methods.lockCollateral(new anchor.BN(lockAmount)).accounts({
      callerProgram: program.programId,
      vault: vaultPda,
      vaultAuthority: vaultAuthorityPda,
    }).rpc();

    const vaultAccountAfter = await program.account.collateralVault.fetch(vaultPda);

    expect(vaultAccountAfter.lockedBalance.toNumber()).to.equal(
      vaultAccountBefore.lockedBalance.toNumber() + lockAmount
    );

    expect(vaultAccountAfter.availableBalance.toNumber()).to.equal(
      vaultAccountBefore.availableBalance.toNumber() - lockAmount
    );

    expect(vaultAccountAfter.totalBalance.toNumber()).to.equal(
      vaultAccountBefore.totalBalance.toNumber()
    );

  })

  it("unlocks collateral", async () => {

    const [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    const before = await program.account.collateralVault.fetch(vaultPda);

    const unlockAmount = 200_000;

    await program.methods.unlockCollateral(
      new anchor.BN(unlockAmount)
    ).accounts({
      callerProgram: program.programId,
      vault: vaultPda,
      vaultAuthority: vaultAuthorityPda,
    }).rpc();

    const after = await program.account.collateralVault.fetch(vaultPda);

    expect(after.lockedBalance.toNumber()).to.equal(
      before.lockedBalance.toNumber() - unlockAmount
    );

    expect(after.availableBalance.toNumber()).to.equal(
      before.availableBalance.toNumber() + unlockAmount
    );

    expect(after.totalBalance.toNumber()).to.equal(
      before.totalBalance.toNumber()
    );

  })

  it("withdraws funds", async () => {

    const [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    const vault = await program.account.collateralVault.fetch(vaultPda);

    const vaultTokenAccount = vault.tokenAccount;
    const mint = vault.mint;

    const userTokenAccount = getAssociatedTokenAddressSync(
      mint,
      user.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID
    )

    const withdrawAmount = 100_000;

    const beforeVault = await program.account.collateralVault.fetch(vaultPda);

    const beforeBalance = await provider.connection.getTokenAccountBalance(
      userTokenAccount
    );

    await program.methods.withdraw(new anchor.BN(withdrawAmount)).accounts({
      user: user.publicKey,
      vault: vaultPda,
      vaultTokenAccount,
      userTokenAccount,
      mintAccount: mint,
      tokenProgram: TOKEN_2022_PROGRAM_ID
    }).rpc()

    const afterVault = await program.account.collateralVault.fetch(vaultPda);

    const afterBalance = await provider.connection.getTokenAccountBalance(
      userTokenAccount
    );

    expect(
      Number(afterBalance.value.amount) - Number(beforeBalance.value.amount)
    ).to.equal(withdrawAmount);

    expect(afterVault.availableBalance.toNumber()).to.equal(
      beforeVault.availableBalance.toNumber() - withdrawAmount
    );

    expect(afterVault.totalBalance.toNumber()).to.equal(
      beforeVault.totalBalance.toNumber() - withdrawAmount
    );

    expect(afterVault.totalWithdrawn.toNumber()).to.equal(
      beforeVault.totalWithdrawn.toNumber() + withdrawAmount
    );

  })
});