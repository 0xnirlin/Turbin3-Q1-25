// raydium.js
import { clusterApiUrl, Connection, Keypair, PublicKey } from '@solana/web3.js';
import { Raydium } from '@raydium-io/raydium-sdk-v2';
import { getMint } from "@solana/spl-token";

async function main() {
    const connection = new Connection(clusterApiUrl("mainnet-beta"), "confirmed");
    const owner = Keypair.generate();
    
    const raydium = await Raydium.load({
        connection,
        owner: owner.publicKey,
        disableLoadToken: false  // Set to false to get token info
    });

    // Get token list and wait for it to load
    const tokenList = await raydium.api.getTokenList();

    console.log(tokenList.mintList[0].address);
    


//     // Get mint info for first token in list
    const mintAccountPublicKey = new PublicKey(tokenList.mintList[0].mintAuthority);
    let mintAccount = await getMint(connection, mintAccountPublicKey);
//     console.log('Mint Account Info:', mintAccount);
}

main().catch(error => {
    console.error('Error occurred:', error);
});