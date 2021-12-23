require('dotenv').config();

const { ApiPromise, WsProvider } = require('@polkadot/api');
const BN = require('bn.js');

const bn1e12 = new BN(10).pow(new BN(12));

const target = [
    [159, 5000],
    [160, 15000],
    [161, 5000],
    [162, 5000],
    [163, 15000],
    [164, 15000],
    [166, 5000],
    [168, 10000],
    [169, 25000],
    [170, 10000],
    [171, 10000],
    [173, 5000],
    [174, 5000],
    [175, 5000],
    [176, 10000],
    [177, 5000],
    [178, 5000],
    [179, 5000],
    [180, 5000],
    [181, 10000],
    [182, 10000],
    [183, 5000],
    [184, 5000],
    [185, 5000],
    [186, 15000],
    [187, 15000],
    [188, 5000],
    [189, 45000	],
    [190, 15000],
    [191, 5000],
    [192, 5000],
    [1621, 5000],
];

async function main() {
    const wsProvider = new WsProvider(process.env.ENDPOINT);
    const api = await ApiPromise.create({ provider: wsProvider });

    const tx = api.tx.utility.batchAll(
        target.map(([pid, amount]) =>
            api.tx.phalaStakePool.contribute(pid, new BN(amount).mul(bn1e12))
        ),
    );

    console.log(tx.toHex());
}

main().catch(console.error).finally(() => process.exit());
