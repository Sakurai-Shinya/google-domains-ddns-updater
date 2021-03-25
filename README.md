# Google Domains DDNS Updater

Version 0.1.1

started by Sakurai Shinya (Twitter:@sakurai_sinya)

This project is japanese only.

***

## 概要

このソフトウェアはGoogle DomainsのダイナミックDNSを自動で更新を行うコンソールアプリケーションです。

Rustで作成されています。（作者はRustの初心者です。）

crontab等のスケジューラを利用してOS起動時や定期的なIPアドレスのチェック・更新に利用できます。

このプログラムはIPv4アドレスの取得を行う為にAWSのサーバに接続します。

http://checkip.amazonaws.com/

（Google Domainsが用意しているIPv4アドレスチェックサイトもあるんですが、IPv6が有効な環境からアクセスするとIPv6アドレスになってしまうので使い物になりません。（AWSさん巻き込んでごめんなさい！！））

Google DomainsのAPIのヘルプは以下を参照してください。

[ダイナミック DNS - Google Domains ヘルプ](https://support.google.com/domains/answer/6147083?hl=ja#zippy=%2Capi-%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%97%E3%81%A6%E3%83%80%E3%82%A4%E3%83%8A%E3%83%9F%E3%83%83%E3%82%AF-dns-%E3%83%AC%E3%82%B3%E3%83%BC%E3%83%89%E3%82%92%E6%9B%B4%E6%96%B0%E3%81%99%E3%82%8)

## 使い方

1. バイナリファイルと同じディレクトリにテキストファイル「domainlist」を作成します。

2. Google DomainsのDNS＞合成レコードで「ダイナミックDNS」を選択し、サブドメインを作成します。

3. 作成したダイナミックDNSの「認証情報を表示」をクリックして、ユーザー名とパスワードを表示します。

4. 1で作成した「domainlist」にカンマ区切りでドメイン,ユーザー名,パスワードを記載します。以下に例を表示します。

```csv
example.com,zHT94GMMi5tcVQCU,tpfdS5BSpLH8JKte
www.example.com,KyHq6z9ebb5gPDuK,RtHuRVqRQ478w5EM
static.example.com,NRtBHCQCRrwGk3Lr,zhTizwUhh8ArK4G6
#でコメントも可能
private.example.com,Yk6YRqgQLXJWbAuc,HsjE7iyFcQJKBakR
admin.example.com,qdiKFfENMp8XTzrc,ebqZXmEcRfsMrQNt
```

5. バイナリファイルを実行します。

6. 更新に成功すると以下のような出力が表示されます。

```
example.comの結果: good 1.2.3.4
www.example.comの結果: good 1.2.3.4
static.example.comの結果: good 1.2.3.4
private.example.comの結果: good 1.2.3.4
admin.example.comの結果: good 1.2.3.4
```

更新作業はマルチスレッドで動作するので応答は順不同です。
このメッセージの詳細は上述されているGoogle Domainsのヘルプを参照してください。

## 依存関係

このプロジェクトは下記の依存関係があります。

[reqwest](https://github.com/seanmonstar/reqwest) (バージョン0.11.2)

## バージョン履歴

2021-03-26 0.1.1 Google Domainsからの応答が911だった場合再試行するよう修正
2021-01-23 0.1.0 初期バージョン公開