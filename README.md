# Binance_TV_rs
Personal challenge, Rewrote Tradingview_Binance_API Project With Rust.

## Environments Variables
```
BINANCE_API="Binance-API-Key" API ของ Binance
BINANCE_SEC="Binance-API-Secret-Key" API ของ Binance
PASSPHARSE="Signal-password" รหัสที่จะส่งมากับ webhook
LINE_TOKEN="LineNotify-Token" รหัส Line Token ได้มาจาก https://notify-bot.line.me
FREE_BALANCE=10 จำนวนเงินขั้นต่ำที่ให้บอทออก Order
```

## TradingView Webhook Messages Example
```
 passphrase = input.string(defval='xxxx', title ='Bot Pass',group='═ Bot Setting ═')
 leveragex  = input.int(125,title='leverage',group='═ Bot Setting ═',tooltip='"NOTHING" to do with Position size',minval=1)
 Alert_OpenLong       = '{"side": "OpenLong", "amount": "@{{strategy.order.contracts}}", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
 Alert_OpenShort      = '{"side": "OpenShort", "amount": "@{{strategy.order.contracts}}", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
 Alert_LongTP         = '{"side": "CloseLong", "amount": "@{{strategy.order.contracts}}", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
 Alert_ShortTP        = '{"side": "CloseShort", "amount": "@{{strategy.order.contracts}}", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
 message_closelong       = '{"side": "CloseLong", "amount": "%100", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
 message_closeshort      = '{"side": "CloseShort", "amount": "%100", "symbol": "{{ticker}}", "passphrase": "'+passphrase+'","leverage":"'+str.tostring(leveragex)+'"}'
```
