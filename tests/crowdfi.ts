import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Crowdfi } from "../target/types/crowdfi";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { randomBytes } from 'node:crypto';
import { Key } from "node:readline";



describe("crowdfi", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfi as Program<Crowdfi>;

  const connection = provider.connection;

  let campaignAdmin: Keypair;
  let campaignerCreator: Keypair;

  let bump;
  let config: PublicKey;
  let campaign: PublicKey;
  let campaignVault: PublicKey;
  
  const seed = new BN(randomBytes(8));

  before(async () => {
    campaignAdmin = anchor.web3.Keypair.generate();
    campaignerCreator = anchor.web3.Keypair.generate();

    await airdrop(connection, campaignAdmin.publicKey, 10);
    await airdrop(connection, campaignerCreator.publicKey, 10);

    [config, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("config"),
      campaignAdmin.publicKey.toBuffer(),
      seed.toArrayLike(Buffer, "le", 8),
    ], program.programId);
    console.log("✅ Config Account Address: ", config);

    [campaign, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("campaign"),
      campaignerCreator.publicKey.toBuffer(),
      Buffer.from("Test Campaign Title"),
    ], program.programId);
    console.log("✅ Campaign Account Address: ", campaign);

    [campaignVault, bump] = PublicKey.findProgramAddressSync([
      Buffer.from("campaign_vault"),
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
      })
      .signers([campaignAdmin])
      .rpc();
    console.log("✅ Your transaction signature", tx);
  });
  
  it("Campaign is Created!", async () => {
    let balance = await getBalance(connection, campaignerCreator.publicKey);
    console.log("💵💵💵 Campaign Creator Balance: ", balance);

    const tx = await program.methods
      .createCampaign(
        "Test Campaign Title", 
        "Description for Test Campaign", 
        "https://test_campaign_initiative.org", 
        new BN(1_000_000), 
        new BN(1_000_000_000), 
        new BN(2_000_000_000))
      .accountsPartial({
        user: campaignerCreator.publicKey,
        config: config,
        campaign: campaign,
        campaignVault: campaignVault,
      })
      .signers([campaignerCreator])
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