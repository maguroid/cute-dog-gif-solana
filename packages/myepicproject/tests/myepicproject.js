const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

describe("myepicproject", () => {
  console.log("🚀 Starting test...");
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it("Is initialized!", async () => {
    const program = anchor.workspace.Myepicproject;
    const baseAccount = anchor.web3.Keypair.generate();
    const tx = await program.rpc.startStuffOff({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log("📝 Your transaction signature", tx);

    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    console.log("👀 GIF Count", account.totalGifs.toString());

    await program.rpc.addGif("insert_a_giphy_link_here", {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("👀 GIF Count", account.totalGifs.toString());

    console.log("👀 GIF List", account.gifList);
  });
});
