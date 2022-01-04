const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Split;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  const baseAccount2 = anchor.web3.Keypair.generate();

  let tx2 = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount2.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount2],
  });

  console.log("📝 Your transaction signature", tx, tx2);
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("📝 Your account", account);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();