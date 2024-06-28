import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
  
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, transfer } from '@solana/spl-token';
// import { getAccount } from '@solana/spl-token';

  
  async function main() {
    console.log("Launching our Dapp...");
  
    const connection = new Connection('http://localhost:8899', 'confirmed');
  
    const payer = Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(
      payer.publicKey,
      LAMPORTS_PER_SOL
    );
  
    await connection.confirmTransaction(airdropSignature);
  
    const mint = await createMint(
        connection,
        payer, 
        payer.publicKey, 
        null, 
        9 
      );
  

    const payerTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey
      );

    await mintTo(
        connection,
        payer, 
        mint,
        payerTokenAccount.address,
        payer.publicKey, 
        1000 * Math.pow(10, 9) 
      );
  
      console.log("Minted 1000 tokens to payer's account");
      let payerAccountInfo = await getAccount(connection, payerTokenAccount.address);
      console.log("Payer's token balance before transfer:", payerAccountInfo.amount.toString());


      
      const recipient = Keypair.generate();
      const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        recipient.publicKey
      );

      await transfer(
        connection,
        payer,
        payerTokenAccount.address,
        recipientTokenAccount.address,
        payer.publicKey, 
        500 * Math.pow(10, 9) 
      );
    
      console.log("Transferred 500 tokens to recipient's account");
      payerAccountInfo = await getAccount(connection, payerTokenAccount.address);
      console.log("Payer's token balance after transfer:", payerAccountInfo.amount.toString());
  }
  
  main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
  );
  