-- 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 is the WETH address

WITH 
    sushi_weth_pairs AS (
        SELECT 
            IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, token1, token0) as token
            , pair as sushi_pool_address
        FROM 
            sushi_ethereum.Factory_evt_PairCreated 
        WHERE
            token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 
            OR token1 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    )
    , uni_weth_pairs AS (
        SELECT 
            IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, token1, token0) as token
            , pair as uni_pool_address
        FROM 
            uniswap_v2_ethereum.Factory_evt_PairCreated 
        WHERE
            token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 
            OR token1 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    )
SELECT 
    uni_weth_pairs.token AS token_address
    , uni_pool_address
    , sushi_pool_address
FROM 
    uni_weth_pairs
JOIN 
    sushi_weth_pairs 
ON 
    uni_weth_pairs.token = sushi_weth_pairs.token