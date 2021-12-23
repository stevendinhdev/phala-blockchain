require('dotenv').config();
const fs = require('fs');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const typedefs = require('@phala/typedefs').khalaDev;

process.env.ENDPOINT = 'ws://127.0.0.1:29945'

async function main() {
    const wsProvider = new WsProvider(process.env.ENDPOINT);
    const api = await ApiPromise.create({ provider: wsProvider, types: typedefs });

    const fHash = await api.rpc.chain.getFinalizedHead();
    const fHead = await api.rpc.chain.getHeader(fHash);
    console.log(fHead.toHuman());

}

main().catch(console.error).finally(() => process.exit());
