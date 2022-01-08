// client.js is used to introduce the reader to generating clients from IDLs.
// It is not expected users directly test with this example. For a more
// ergonomic example, see `tests/basic-0.js` in this workspace.

const anchor = require("@project-serum/anchor");
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

  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId, provider);

  // Execute the RPC.
  await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      authority: baseAccount.publicKey
    },
    signers: [baseAccount]
  });
  // #endregion main
}

console.log("Running client.");
main().then(() => console.log("Success"));
