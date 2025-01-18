import wallet from "./wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = await readFile("/home/nirlin/coding/turbine/Turbin3-Q1-25/solana-starter/ts/cluster1/generug.png");
        const imageGeneric = createGenericFile(image, "generug.png");
        const imageUri = "https://devnet.irys.xyz/5q55rtLBtzWdytZnBgUFQVPe8Qphz6ZKCgZVEZgy3UYf"

        const metadata = {
            name: "Nirlin Rug",
            symbol: "RUG",
            description: "A beautiful rug created by Nirlin",
            image: imageUri,
            attributes: [
                {trait_type: 'Style', value: 'Modern'},
                {trait_type: 'Material', value: 'Digital'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageUri
                    },
                ]
            },
            creators: []
        };

        const metadataGeneric = createGenericFile(
            Buffer.from(JSON.stringify(metadata)),
            'metadata.json'
        );

        const [myUri] = await umi.uploader.upload([metadataGeneric]);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
