# IDIS ファイルシステムについて
idisは独自のファイルシステムをdbとosが提供するファイルシステムによって構築します。  
dbにメタデータもしくは小規模なバイナリを保存し、os提供のファイルシステムにバイナリを保存します。  
これにより
- レスポンスの高速化
- ユニーク性の保証
- セキュリティの向上
- 容易な拡張性

を実現します。

## ディレクトリ構造
ユーザーごとに独立したファイルシステムを構成します。基本ディレクトリは以下のとおりです  

* `<user>`
* /agent
  * `<agent>`.html
* /etc
  * agent_register.json
  * path_register.json
* /home
  * about_user.json
  * `<any file & folder>`
* /var
  * user_log.json

agentはユーザー定義のアプリケーションです。
また`/etc`にはagentやpathの構造を指定するレジスターファイルを設置します。  


またAPI定義では`/get/<user>/<path>`
となっていますがここでの`<path>`は`</home/>`以下に自動転送されます。

ex. `<origin>/get/<user>/about_user.json` -> `<user>`のプロフィール情報