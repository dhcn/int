订阅要关注头部流动性供给大的市场，流动性大的市场对市场价格的影响力更大。
## Phemex
> https://phemex.com/cn/contract/index-price

Phemex的指数价格来自六个现货交易所的加权计算：Coinbase，Bitstamp，Kraken，Binance，OKex和Huobi。例如，关于BTC / USD，我们直接从Coinbase，Bitstamp和Kraken得出实时交易价格。关于BTC / USDT，将根据USDT / USD价格，在Binance，OKex和Huobi的价格转换为BTC / USD。如果这些交易所中的任何一个在15秒内未能更新其价格（部分交易量较小币种可能会延迟至60S），它将被暂时从计算中删除。在计算平均值之前，还会删除最高价和最低价。更新指数价格至少需要三个合格价格源，否则它将保持上一个价格。

## Bybit
> https://help.bybit.com/hc/zh-cn/articles/360039261094-%E6%8C%87%E6%95%B0%E4%BB%B7%E6%A0%BC%E8%AE%A1%E7%AE%97

指数价格由预估指数价格.ECOIN(BTC/ETH/XRP/EOS) 计算得出的，以下通过BTC的指数价格计算来举例。

.EBTC是这些交易所：Bitstamp、CoinbasePro、Kraken、Gemini和Bittrex的加权BTC现货交易价格之和。加权均数（交易量权重）是各个交易所上一个月的月交易量。

## CoinFlex
> https://v2api.coinflex.com/v2/all/markets
