const { pruntime_rpc: { PhactoryAPI } } = require('./proto/pruntime_rpc');

export const createPRuntimeApi = (endpoint) => {
    return PhactoryAPI.create(
        async (method, requestData, callback) => {
            const url = `${endpoint}/prpc/PhactoryAPI.${method.name}`
            logger.debug({ url, requestData }, 'Sending HTTP request...')
            try {
                const res = await clientQueue.add(() =>
                    promiseRetry(
                        (retry) => requestQueue
                            .add(() =>
                                fetch(url, {
                                    method: 'POST',
                                    body: requestData,
                                    headers: {
                                        'Content-Type': 'application/octet-stream',
                                    },
                                })
                            )
                            .catch((...args) => {
                                logger.warn(...args)
                                return retry(...args)
                            }),
                        {
                            retries: 3,
                            minTimeout: 1000,
                            maxTimeout: 30000,
                        }
                    )
                )

                const buffer = await res.buffer()
                if (res.status === 200) {
                    callback(null, buffer)
                } else {
                    const errPb = prpc.PrpcError.decode(buffer)
                    logger.warn(prpc.PrpcError.toObject(errPb))
                    callback(new Error(errPb.message))
                }
            } catch (e) {
                callback(e)
            }
        },
        false,
        false
    )
};
