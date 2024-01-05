import * as web3 from "@solana/web3.js";
// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey("BmEbNu6JkC21UnTuF553rDbxCkeBhT3tCATuC8Z6ZTpJ");
const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
const wallet = { keypair: web3.Keypair.generate() };

// Client
console.log("My address:", wallet.keypair.publicKey.toString());
const balance = await connection.getBalance(wallet.keypair.publicKey);
console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);


const CREATE_PDA_OPCODE = 0;
const INCREMENT_COUNTER_OPCODE = 1;
const programId = await PROGRAM_ID;


 // Derive the PDA
const [pda, bumpSeed] = web3.PublicKey.findProgramAddressSync(
  [wallet.keypair.publicKey.toBuffer()],
  programId
);


// Check if the PDA account already exists
const pdaAccountInfo = await connection.getAccountInfo(pda);
const transaction = new web3.Transaction()
if (!pdaAccountInfo) {
  console.log("PDA account does not exist, creating it")
  // PDA account does not exist, create it
  transaction.add(
    {
      keys: [
        { pubkey: wallet.keypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: pda, isSigner: false, isWritable: true },
        { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId,
      data: Buffer.from([CREATE_PDA_OPCODE]), // Opcode for creating PDA
    }
  );
  console.log(`Added transaction to create PDA account...`);

} else {
  console.log("PDA account exists")
}

// Increment the counter twice
for (let i = 0; i < 2; i++) {
  transaction.add(
    new web3.TransactionInstruction({
      keys: [
        { pubkey: wallet.keypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: pda, isSigner: false, isWritable: true },
      ],
      programId,
      data: Buffer.from([INCREMENT_COUNTER_OPCODE]), // Opcode for incrementing counter
    })
  );

  console.log(`Added transaction to increment counter (${i + 1})...`);
}

let txHash = await web3.sendAndConfirmTransaction(
  connection,
  transaction,
  [wallet.keypair],
);
console.log("Transaction sent with hash:", txHash);

