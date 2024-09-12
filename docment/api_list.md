## HTTP Endpoints

### `/users/<userIDquery>` -> JSON
- **Description:** 全ユーザーIDを検索し、ヒットするユーザーとフォロワー数をJSON形式で返却します。
- **Recommendation:** WS-APIを推奨。

### `/user/<userID>` -> JSON
- **Description:** 指定したユーザーIDのユーザー情報を取得します。
- **Recommendation:** WS-APIを推奨。

### `/ls/@<userID>/<path>` -> JSON
- **Description:** 指定したパスのフォルダまたはファイルのメタデータを取得します。
- **Recommendation:** WS-APIを推奨。

### `/rm/@<userID>/<path>` -> JSON
- **Description:** 指定したパスのフォルダまたはファイルを削除し、メタデータを取得します。
- **Recommendation:** WS-APIを推奨。

### `/upload/@<userID>/<path>` -> JSON
- **Description:** ファイルをアップロードするか、フォルダを作成します。
- **Recommendation:** WS-APIを強く推奨。

### `/edit/@<userID>/<path>` -> JSON
- **Description:** ファイルまたはフォルダのメタデータを上書きします。
- **Recommendation:** WS-APIを強く推奨。

### `/get/@<userID>/<path>` -> BinaryStream
- **Description:** 指定したファイルを強制ダウンロードします。

### `/viw/@<userID>/<path>` -> BinaryStream
- **Description:** 指定したファイルを取得します。

### `/@<user>/<path>` -> HTML
- **Description:** エージェントを用いてファイルを取得します。

### `/resend?url=<URL>&ms=<Message>&time=<WaitTime>` -> HTML
- **Description:** リダイレクトページを取得します。

### `/login` -> HTML
- **Description:** ログインページ。

### `/signup` -> HTML
- **Description:** サインアップページ。

### `/root/r` -> BinaryStream
- **Description:** システムのスタックファイルを取得します。