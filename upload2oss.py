#!/usr/bin/env python

import datetime
import os

def exec_cmd(cmd_str):
    result = os.system(cmd_str)
    if result == 0:
        print("exec success:" + cmd_str)
    else:
        print("exec error:" + cmd_str)
        exit()

def cp_all():
    date = datetime.date(2020, 5, 25);
    while True:
        cmd_str = "./ossutil64 cp oss://int-data/OkexSubscriber%s.tar.gz oss://int-data/Okex/" % date.strftime(
            "%Y-%m-%d")
        print(cmd_str)
        print(os.system(cmd_str))
        date = date + datetime.timedelta(1)
        if date > datetime.date(2021, 1, 1):
            break;


def upload_yesterday():
    now = datetime.date.today()
    yesterday = now - datetime.timedelta(1)
    yesterday = yesterday.strftime("%Y-%m-%d")
    os.chdir("./dev_codes/int")# because crontab python script exec path is the user directory path ~
    exchanges = ["Binan", "Huobi","Bybit","Bybiti","Bybite","Phemex","Phemexe","CoinFlexe","FTX","Bitmex","Coinbase","Kraken","Bitstamp"]#"Okex","Converge","Bitfinex",,

    for exchange in exchanges:
        filename = "%sSubscriber%s" % (exchange, yesterday)
        compress_cmd = "tar -cvzf %s.tar.gz %s" % (filename, filename)
        exec_cmd(compress_cmd)
        upload_cmd = "./ossutil64 cp %s.tar.gz oss://int-data/%s/" % (filename, exchange)
        exec_cmd(upload_cmd)
        rm_cmd = "rm -rf %s*" % (filename)
        exec_cmd(rm_cmd)



if __name__ == '__main__':
    upload_yesterday()

