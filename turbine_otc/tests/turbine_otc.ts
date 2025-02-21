import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TurbineOtc } from "../target/types/turbine_otc";
import { Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";
import { expect } from "chai";

describe("turbine_otc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TurbineOtc as Program<TurbineOtc>;
  const provider = anchor.getProvider();

  // Test accounts
  let configPda: PublicKey;
  let otcOrderPda: PublicKey;
  let maker: Keypair;
  let taker: Keypair;
  let offerMint: PublicKey;
  let askMint: PublicKey;
  let makerOfferAccount: PublicKey;
  let makerAskAccount: PublicKey;
  let takerOfferAccount: PublicKey;
  let takerAskAccount: PublicKey;

  before(async () => {
    // Create test accounts
    maker = Keypair.generate();
    taker = Keypair.generate();

    // Airdrop SOL to maker and taker
    await provider.connection.requestAirdrop(maker.publicKey, 2e9);
    await provider.connection.requestAirdrop(taker.publicKey, 2e9);

    // Create test tokens
    offerMint = await createMint(
      provider.connection,
      maker,
      maker.publicKey,
      null,
      9
    );
    askMint = await createMint(
      provider.connection,
      maker,
      maker.publicKey,
      null,
      9
    );

    // Create token accounts
    makerOfferAccount = await createAccount(
      provider.connection,
      maker,
      offerMint,
      maker.publicKey
    );
    makerAskAccount = await createAccount(
      provider.connection,
      maker,
      askMint,
      maker.publicKey
    );
    takerOfferAccount = await createAccount(
      provider.connection,
      taker,
      offerMint,
      taker.publicKey
    );
    takerAskAccount = await createAccount(
      provider.connection,
      taker,
      askMint,
      taker.publicKey
    );

    // Mint some tokens to maker and taker
    await mintTo(
      provider.connection,
      maker,
      offerMint,
      makerOfferAccount,
      maker.publicKey,
      1000e9
    );
    await mintTo(
      provider.connection,
      maker,
      askMint,
      takerAskAccount,
      maker.publicKey,
      1000e9
    );

    // Derive PDAs
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("turbine_config")],
      program.programId
    );

    [otcOrderPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("otc order"),
        maker.publicKey.toBuffer(),
        offerMint.toBuffer(),
        askMint.toBuffer(),
      ],
      program.programId
    );
  });

  it("Initialize config", async () => {
    // Add required parameters for initConfig
    const feePercentage = 100; // 1%
    const maxFeePercentage = 1000; // 10%
    const minFeePercentage = 50; // 0.5%
    const maxPremium = 2000; // 20%
    const minPremium = 100; // 1%
    const feeCollector = provider.publicKey;
    const maxDuration = 86400; // 1 day in seconds

    const tx = await program.methods
      .initConfig(
        feePercentage,
        maxFeePercentage,
        minFeePercentage,
        maxPremium,
        minPremium,
        feeCollector,
        maxDuration
      )
      .accounts({
        creator: provider.publicKey,
        turbineConfig: configPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Init config transaction signature", tx);

    // Verify config account
    const configAccount = await program.account.config.fetch(configPda);
    expect(configAccount.owner).to.eql(provider.publicKey);
  });

  it("Make OTC order", async () => {
    const offerAmount = new anchor.BN(100e9);
    const seed = new anchor.BN(Date.now()); // Unique seed
    const seller = maker.publicKey;
    const expiryTimestamp = new anchor.BN(Math.floor(Date.now() / 1000) + 3600); // 1 hour from now
    const premium = 500; // 5%

    const tx = await program.methods
      .makeOtcOrder(
        offerAmount,
        seed,
        seller,
        expiryTimestamp,
        premium
      )
      .accounts({
        buyer: maker.publicKey,
        tokenMint: offerMint,
        otcOrder: otcOrderPda,
        makerTokenAccount: makerOfferAccount,
        turbineConfig: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([maker])
      .rpc();

    console.log("Make OTC order transaction signature", tx);

    // Verify OTC order account
    const orderAccount = await program.account.order.fetch(otcOrderPda);
    expect(orderAccount.seller).to.eql(maker.publicKey);
    expect(orderAccount.tokenMint).to.eql(offerMint);
    expect(orderAccount.amount.eq(offerAmount)).to.be.true;
  });

  it("Take OTC order", async () => {
    const amountSol = new anchor.BN(1e9); // 1 SOL

    const tx = await program.methods
      .takeOtcOrder(amountSol)
      .accounts({
        seller: maker.publicKey,
        buyer: taker.publicKey,
        otcOrder: otcOrderPda,
        buyerTokenAccount: takerOfferAccount,
        sellerTokenAccount: makerAskAccount,
        turbineConfig: configPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([taker])
      .rpc();

    console.log("Take OTC order transaction signature", tx);

    // Add balance verification here
  });
});
