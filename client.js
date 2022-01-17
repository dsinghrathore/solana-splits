// client.js is used to introduce the reader to generating clients from IDLs.
// It is not expected users directly test with this example. For a more
// ergonomic example, see `tests/basic-0.js` in this workspace.

const anchor = require("@project-serum/anchor");
const { Keypair } = require("@solana/web3.js");
const { expect } = require("chai");
const { SystemProgram } = anchor.web3;
const idl = require("./target/idl/split.json");
const provider = anchor.Provider.env();
// Configure the local cluster.
anchor.setProvider(provider);

async function main() {
  // #region main
  // Read the generated IDL.
  const programId = new anchor.web3.PublicKey(
    "4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr"
  );
  // const baseAccount = anchor.web3.Keypair.generate();
  const splitAdmin = anchor.web3.Keypair.generate();
  // const splitAccount = anchor.web3.Keypair.generate();
  const aone = anchor.web3.Keypair.generate();
  const atwo = anchor.web3.Keypair.generate();
  // const aone = new anchor.web3.PublicKey("4m1eWNndyhE8eJyQcde8MYMV3tzP4wS5xsARZQTHAKpo");
  // const atwo = new anchor.web3.PublicKey("BfPHFPUCzLukQBRrdrK3eDbzYB8cG58SCg8FhT3jvurX");
  const athree = anchor.web3.Keypair.generate();
  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId, provider);

  // console.log(aone.publicKey.toString(), atwo.publicKey.toString());
  const [baseAccount, baseAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(provider.wallet.publicKey.toString().substring(0, 6)),
        provider.wallet.publicKey.toBuffer(),
      ],
      programId
    );
  // Execute the RPC.
  console.log(
    `slice:${Buffer.from(
      provider.wallet.publicKey.toString().substring(0, 6)
    )}bump:${baseAccountBump} baseA:${baseAccount}`
  );
  let initialize_tx = await program.rpc.initialize(
    new anchor.BN(baseAccountBump),
    {
      accounts: {
        baseAccount: baseAccount,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        authority: provider.wallet.publicKey,
      },
    }
  );

  console.log("📝 Your transaction signature", initialize_tx);
  // #endregion main
  // let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  // console.log("🤺 Your account ", account);
  // console.log(
  //   "Pks",
  //   aone.publicKey.toString(),
  //   atwo.publicKey.toString(),
  //   athree.publicKey.toString()
  // );
  console.log("Running client.");
  try {
    // let [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    //   [Buffer.from("0")],
    //   programId
    // );
    // // anchor.web3.PublicKey.findProgramAddress()
    // console.log(`bump:${bump} pubkey: ${pda.toBase58()}`);
    // let new_split_1 = await program.rpc.newSplit(
    //   [new anchor.BN(60), new anchor.BN(40)],
    //   [provider.wallet.publicKey, atwo.publicKey],
    //   {
    //     accounts: {
    //       baseAccount: baseAccount.publicKey,
    //       splitAccount: pda,
    //       user: provider.wallet.publicKey,
    //       systemProgram: SystemProgram.programId,
    //       authority: baseAccount.publicKey,
    //     },
    //     signers: [baseAccount],
    //   }
    // );
    // let new_split_2 = await program.rpc.newSplit(
    //   [new anchor.BN(60), new anchor.BN(40)],
    //   [aone.publicKey, athree.publicKey],
    //   {
    //     accounts: {
    //       baseAccount: baseAccount.publicKey,
    //       user: provider.wallet.publicKey,
    //       systemProgram: SystemProgram.programId,
    //     },
    //   }
    // );
    // console.log("📝 New Split 1", new_split_1);
    // // console.log(program.account);
    // let split_account_info = await program.account.splitAccount.fetch(
    //   splitAccount.publicKey
    // );
    // let base_account_info = await program.account.baseAccount.fetch(
    //   baseAccount.publicKey
    // );
    // console.log("Split acc:", split_account_info);
    // console.log("base acc:", base_account_info);
    //   let base_account_info = await program.account.baseAccount.fetch(
    //     baseAccount.publicKey
    //   );
    //   expect(base_account_info.splits.length).is.equal(2);
    //   // GET A PDA
    // let [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
    //   [Buffer.from("test")],
    //   programId
    // );
    // console.log(`bump: ${bump}, pubkey: ${pda.toBase58()}`);
    // let send_sol_tx = await program.rpc.sendSol(
    //   new anchor.BN(0),
    //   new anchor.BN(100000),
    //   {
    //     accounts: {
    //       baseAccount: baseAccount.publicKey,
    //       user: provider.wallet.publicKey,
    //       systemProgram: SystemProgram.programId,
    //       pdaAccount: pda,
    //       splitAccount:splitAccount.publicKey
    //     },
    //   }
    // );
    //   console.log("📝 Sent Sol", send_sol_tx);
    //   try {
    //     let withdraw_tx = await program.rpc.withdraw(
    //       new anchor.BN(0),
    //       new anchor.BN(0),
    //       {
    //         accounts: {
    //           baseAccount: baseAccount.publicKey,
    //           msgSender: provider.wallet.publicKey,
    //           systemProgram: SystemProgram.programId,
    //           pdaAccount: pda,
    //           receiver: atwo.publicKey,
    //         },
    //       }
    //     );
    //     console.log("📝 withdrew Sol", withdraw_tx);
    //     console.log("🚀🚀🚀 LFG!!!! ");
    //   } catch (error) {
    //     console.log("Error 🟥", error);
    //   }
    //   console.log("Done!");
  } catch (error) {
    console.log("Error 🟥", error);
  }
}
main().then(() => console.log("Success"));

// USING PDAs
// 1. Create a PDA for new Split
// 2. Send SOL to the PDA
// 3. Use the PDA as signer to access withdraw fn

// ^ scratch that
