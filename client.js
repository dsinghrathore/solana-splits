// client.js is used to introduce the reader to generating clients from IDLs.
// It is not expected users directly test with this example. For a more
// ergonomic example, see `tests/basic-0.js` in this workspace.

const anchor = require("@project-serum/anchor");
const { expect } = require("chai");
const { SystemProgram } = anchor.web3;
const idl = require("./idl.json")
const provider = anchor.Provider.env();
// Configure the local cluster.
anchor.setProvider(provider);

async function main() {
  // #region main
  // Read the generated IDL.
  const programId = new anchor.web3.PublicKey("4tzDAD5KLntPhT8t3gjqs85vsT5aguZTNCoeRvKkt5zr");
  const baseAccount = anchor.web3.Keypair.generate();
  const splitAdmin = anchor.web3.Keypair.generate();
  const aone = anchor.web3.Keypair.generate();
  const atwo = anchor.web3.Keypair.generate();
  // const aone = new anchor.web3.PublicKey("4m1eWNndyhE8eJyQcde8MYMV3tzP4wS5xsARZQTHAKpo");
  // const atwo = new anchor.web3.PublicKey("BfPHFPUCzLukQBRrdrK3eDbzYB8cG58SCg8FhT3jvurX");
  const athree = anchor.web3.Keypair.generate();
  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId, provider);

  // console.log(aone.publicKey.toString(), atwo.publicKey.toString());

  // Execute the RPC.
  let initialize_tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      authority: baseAccount.publicKey
    },
    signers: [baseAccount]
  });

  console.log("📝 Your transaction signature", initialize_tx);
  // #endregion main
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("🤺 Your account ", account);

  try {
    let new_split_1 = await program.rpc.newSplit(
      aone.publicKey,
      [new anchor.BN(60), new anchor.BN(40)],
      [aone.publicKey, atwo.publicKey],
      {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        }
      }
    );

    let new_split_2 = await program.rpc.newSplit(
      aone.publicKey,
      [new anchor.BN(60), new anchor.BN(40)],
      [aone.publicKey, athree.publicKey],
      {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        }
      }
    );

    console.log("📝 New Split 1", new_split_1);
    console.log("📝 New Split 2", new_split_2);

    let base_account_info = await program.account.baseAccount.fetch(baseAccount.publicKey);
    expect(base_account_info.splits.length).is.equal(2);

    let send_sol_tx = await program.rpc.sendSol(
      new anchor.BN(0),
      new anchor.BN(2),
      {
        accounts: {
          baseAccount: baseAccount.publicKey,
          msgSender: provider.wallet.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId
        },
        remainingAccounts: []
      }
    );

    console.log("📝 Sent Sol", send_sol_tx);
  } catch(e){
    console.log("Error 🟥", e);
  }

  console.log("Done!");
}

console.log("Running client.");
main().then(() => console.log("Success"));
