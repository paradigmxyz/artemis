-- 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 is the WETH address

WITH 
    uniV3_weth_pairs AS (
        SELECT 
            IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, token1, token0) as token
            , IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, true, false) as weth_token0
            , pool as pair
            , fee
        FROM 
            uniswap_v3_ethereum.Factory_evt_PoolCreated 
        WHERE
            token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 
            OR token1 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    )
    , uniV2_weth_pairs AS (
        SELECT 
            IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, token1, token0) as token
            , IF(token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2, true, false) as weth_token0
            , pair
        FROM 
            uniswap_v2_ethereum.Factory_evt_PairCreated 
        WHERE
            token0 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 
            OR token1 = 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
    )
SELECT 
    uniV3_weth_pairs.token AS token_address
    , uniV3_weth_pairs.pair AS v3_pool
    , uniV2_weth_pairs.pair AS v2_pool
    , uniV3_weth_pairs.weth_token0 AS weth_token0
FROM 
    uniV3_weth_pairs
JOIN 
    uniV2_weth_pairs 
ON 
    uniV3_weth_pairs.token = uniV2_weth_pairs.token