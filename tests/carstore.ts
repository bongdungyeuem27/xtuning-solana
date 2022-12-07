import { Keypair } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { Carstore } from "../target/types/carstore";
import { expect } from "chai";

describe("carstore", async () => {
  const amount = anchor.web3.LAMPORTS_PER_SOL * 100;
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Carstore as anchor.Program<Carstore>;

  const provider2 = anchor.AnchorProvider.env();
  anchor.setProvider(provider2);

  const program2 = anchor.workspace.Carstore as anchor.Program<Carstore>;

  const [carTypeAccount, carTypeAccountBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("car_type_account")],
      program.programId
    );

  const [carAccount, carAccountBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("car_account")],
      program.programId
    );

  const [userAccount, userAccountBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("user_account")],
      program.programId
    );

  const [carStoreAccount, carStoreAccountBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("car_store_account")],
      program.programId
    );

  it("Is initialized!", async () => {
    await program.methods
      .initialize(
        carTypeAccountBump,
        carAccountBump,
        userAccountBump,
        carStoreAccountBump
      )
      .accounts({
        carTypeAccount: carTypeAccount,
        carAccount: carAccount,
        userAccount: userAccount,
        carStoreAccount: carStoreAccount,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    const carStoreAccountData = await program.account.carStoreAccount.fetch(
      carStoreAccount
    );

    expect(
      carStoreAccountData.owner.equals(provider.wallet.publicKey),
      "error start"
    ).to.true;
  });
  it("add car type", async () => {
    const carTypeName = "a";
    await program.methods
      .addCarType(carTypeName, "aasdasd", "bmw", new anchor.BN(amount))
      .accounts({
        user: provider.wallet.publicKey,
        carTypeAccount: carTypeAccount,
      })
      .rpc();
    const carTypeAccountData = await program.account.carTypeAccount.fetch(
      carTypeAccount
    );
    expect(carTypeAccountData.list[0].carTypeName, "not car type").to.equal(
      carTypeName
    );
  });
  it("test car", async () => {
    await program.methods
      .addCar(new anchor.BN(0))
      .accounts({
        user: provider.wallet.publicKey,
        carAccount: carAccount,
        carStoreAccount: carStoreAccount,
      })
      .rpc();

    const carAccountData = await program.account.carAccount.fetch(carAccount);
    expect(carAccountData.list[0].carAvailable, "not car").to.equal(true);
  });

  it("test user", async () => {
    await program.methods
      .addUser("k", "lamdong")
      .accounts({
        user: provider2.wallet.publicKey,
        userAccount: userAccount,
      })
      .rpc();
    const userAccountData = await program.account.userAccount.fetch(
      userAccount
    );
    expect(userAccountData.list[0].userName, "not user").to.equal("k");
  });

  it("test buy", async () => {
    const wallet1Balance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );
    console.log(wallet1Balance / anchor.web3.LAMPORTS_PER_SOL);
    const [event, slot]: any = await new Promise(async (resolve, reject) => {
      let listener = program.addEventListener("MyEvent", (event, slot) => {
        resolve([event, slot]);
      });
      await program.methods
        .buyCar(new anchor.BN(0))
        .accounts({
          payerAccount: provider2.wallet.publicKey,
          reciverAccount: provider.wallet.publicKey,
          carStoreAccount: carStoreAccount,
          userAccount: userAccount,
          carAccount: carAccount,
          carTypeAccount: carTypeAccount,
        })
        .rpc()
        .catch((error) => reject(error));
      program.removeEventListener(listener);
    });

    const wallet2Balance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );
    console.log(wallet2Balance / anchor.web3.LAMPORTS_PER_SOL);
    // console.log("event", event);
    // console.log("slot", slot);
  });
});
