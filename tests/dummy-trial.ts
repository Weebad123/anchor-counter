//import idl from "./idl.json";
/*
import idl from "./idl.json";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { Program, Idl, AnchorProvider, setProvider } from "@coral-xyz/anchor";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

const { connection } = useConnection();

const wallet = useAnchorWallet();

const provider = new AnchorProvider(connection, wallet, {});
setProvider(provider);

// specify explicitly the program id
const programId = new PublicKey("JPLockxtkngHkaQT5AuRYow3HyUv5qWzmhwsCPd653n");
const program = new Program(idl as Idl, programId);

// we utilise the await program.methods.instructionName().accounts({}).rpc()
// await program.methods.instructionName().accounts({}).transaction();  await sendTransaction(transaction, connection);

// first case
const transaction = await program.methods
    .instructionName()
    .accounts({})
    .rpc();

// 2nd case, returning transaction Object directly without rpc usage
const transaction0 = await program.methods
    .instructionName()
    .accounts({})
    .transaction();

await sendTransaction(transaction0, connection);

// 3rd case, returning Instruction object
const instructionOne = await program.methods
    .instructionOneName(instructionOneDataInputs)
    .accounts({})
    .instruction();

const instructionTwo = await program.methods
    .instructionTwoName(instructionTwoDataInputs)
    .accounts({})
    .instruction();

const transaction2 = new Transaction().add(instructionOne, instructionTwo);

await sendTransaction(transaction2, connection);


// Fetching program Accounts in Account
// # call account on program, and specify the particular account, with .all flag... returns all the accounts that implements the specified Account type
const accounts = await program.account.counter.all();
// # we can apply a filter to .all method by calling memcmp, with an offset and bytes args
const accounts = await program.account.counter.all([
    {
        memcmp: {
            offset: 8,
            bytes: bs58.encode(new BN(0, "le").toArray()),
        },
    },
]);

// if we know the address we're looking for, we just apply the .fetch method passing in the public key of the account
const account = await program.account.counter.fetch(ACCOUNT_ADDRESS);

// if we wanna fetch multiple accounts all whose addresses are known, we do so with fetchMultiple method
const account = await program.account.counter.fetchMultiple([
    ACCOUNT_ADDRESS_ONE,
    ACCOUNT_ADDRESS_TWO,
]);
*/

