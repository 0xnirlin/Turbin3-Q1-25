// raydium.js
import { Connection, Keypair } from '@solana/web3.js';
import { Raydium } from '@raydium-io/raydium-sdk-v2';

async function main() {
    const connection = new Connection('https://api.mainnet-beta.solana.com');
    const owner = Keypair.generate();
    
    const raydium = await Raydium.load({
        connection,
        owner: owner.publicKey,
        disableLoadToken: false  // Set to false to get token info
    });

    // Get token list and wait for it to load
    const tokenList = await raydium.api.getTokenList();
    
    // Debug token list
    console.log('Raw token list:', tokenList);
    
    // Ensure we have data before proceeding
    if (!tokenList) {
        console.error('Failed to load token list');
        return;
    }

    // Convert tokenList to array if it's not already
    const tokenArray = Array.isArray(tokenList) ? tokenList : Object.values(tokenList);

    // Debug first few tokens
    console.log('First few tokens:', tokenArray.slice(0, 3));
    
    // Find specific tokens (e.g., USDC, SOL, BONK)
    const usdc = tokenArray.find(token => token?.symbol === 'USDC');
    const sol = tokenArray.find(token => token?.symbol === 'SOL');
    const bonk = tokenArray.find(token => token?.symbol === 'BONK');

    console.log('USDC Token:', usdc);
    console.log('SOL Token:', sol);
    console.log('BONK Token:', bonk);

    // You can also search by name
    const rayToken = tokenArray.find(token => token?.name === 'Raydium');
    console.log('Raydium Token:', rayToken);

    // List all valid tokens with their mints
    console.log('\nAll Tokens:');
    tokenArray.forEach(token => {
        if (token && token.symbol && token.mint) {
            console.log(`${token.symbol}: ${token.mint}`);
        }
    });
}

main().catch(error => {
    console.error('Error occurred:', error);
});