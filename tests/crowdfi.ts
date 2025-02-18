import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Crowdfi } from "../target/types/crowdfi";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { randomBytes } from 'node:crypto';

describe("crowdfi", () => {
  // Configure the client to use the local cluster.
  // remember to change this to a const
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfi as Program<Crowdfi>;
  const connection = provider.connection;

  let campaignAdmin: Keypair;
  let campaignCreator: Keypair;

  let config: PublicKey;
  let config_bump: Number;
  let campaign: PublicKey;
  let campaign_bump: Number;
  let campaignVault: PublicKey;
  let vault_bump: Number;
  
  const seed = new BN(randomBytes(8));

  before(async () => {
    campaignAdmin = anchor.web3.Keypair.generate();
    campaignCreator = anchor.web3.Keypair.generate();

    console.log("👨🏽‍🔧 Campaign Admin: ", campaignAdmin.publicKey);
    console.log("👨🏽‍🔧 Campaign Creator: ", campaignCreator.publicKey);

    await airdrop(connection, campaignAdmin.publicKey, 10);
    await airdrop(connection, campaignCreator.publicKey, 10);

    [config, config_bump] = PublicKey.findProgramAddressSync([
      // Buffer.from("config"),
      anchor.utils.bytes.utf8.encode("config"),
      campaignAdmin.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, "le", 8),
    ], program.programId);
    console.log("✅ Config Account Address: ", config);

    [campaign, campaign_bump] = PublicKey.findProgramAddressSync([
      // Buffer.from("campaign"),
      anchor.utils.bytes.utf8.encode("campaign"),
      // campaignCreator.publicKey.toBuffer(),
      // Buffer.from("Test Campaign Title"),
    ], program.programId);
    console.log("✅ Campaign Account Address: ", campaign);

    [campaignVault, vault_bump] = PublicKey.findProgramAddressSync([
      // Buffer.from("campaign_vault"),
      anchor.utils.bytes.utf8.encode("campaign_vault"),
      campaign.toBuffer(),
    ], program.programId);
    console.log("✅ Campaign Vault Account Address: ", campaignVault);
  });

  it("Campaign Config is Initialized!", async () => {
    const tx = await program.methods
      .initialize(seed, new BN(1_000), new BN(1_000))
      .accountsPartial({
        admin: campaignAdmin.publicKey,
        config: config,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([campaignAdmin])
      .rpc();
    console.log("✅ Your transaction signature", tx);
  });
  
  // it("Campaign is Created!", async () => {
  //   let balance = await getBalance(connection, campaignCreator.publicKey);
  //   console.log("💵💵💵 Campaign Creator Balance: ", campaignCreator.publicKey, balance / LAMPORTS_PER_SOL);
    
  //   const tx = await program.methods
  //     .createCampaign(
  //       "Test Campaign Title", 
  //       "Description for Test Campaign", 
  //       "https://test_campaign_initiative.org", 
  //       new BN(1_000_000), 
  //       new BN(1_000_000_000), 
  //       new BN(2_000_000_000))
  //     .accounts({
        
  //       user: campaignCreator.publicKey,
  //       // config: config,
  //       campaign: campaign,
  //       // campaignVault: campaignVault,
  //       systemProgram: SystemProgram.programId,
  //     })
  //     .signers([campaignCreator])
  //     .rpc();
  //   console.log("✅ Your transaction signature", tx);
  // });
  
  it("Campaign V2 is Created!", async () => {
    let balance = await getBalance(connection, campaignCreator.publicKey);
    console.log("💵💵💵 Campaign Creator Balance: ", campaignCreator.publicKey, balance / LAMPORTS_PER_SOL);
    
    const tx = await program.methods
      .createCampaignV2()
      .accounts({
        user: campaignAdmin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([campaignAdmin])
      .rpc();
    console.log("✅ Your transaction signature", tx);
  });
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}