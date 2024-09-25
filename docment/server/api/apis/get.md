/get API

概要

/getエンドポイントは、ユーザー名とパスを指定してデータを取得するAPIです。このエンドポイントでは、バイナリデータやJSONデータを返却します。

新しい仕様として、userとpathを含むリクエスト全体をJSON形式で送信することが可能になりました。これにより、複雑なクエリを含むリクエストを簡潔かつ柔軟に行うことができます。また、セキュリティの観点からも、JSON形式でのクエリ送信（POSTリクエスト）を推奨します。


---

使用されるJSONデータ

以下のJSONデータを例として使用します。このデータに対してフィールドの指定や範囲の指定を行います。

{
  "id": 12345,
  "name": "John Doe",
  "email": "johndoe@example.com",
  "age": 30,
  "is_active": true,
  "address": {
    "street": "1234 Elm St",
    "city": "Somewhere",
    "state": "CA",
    "postal_code": "90001"
  },
  "phone_numbers": [
    {
      "type": "home",
      "number": "555-555-1234"
    },
    {
      "type": "work",
      "number": "555-555-5678"
    }
  ],
  "preferences": {
    "contact_method": "email",
    "newsletter_subscribed": true
  }
}


---

エンドポイント構造

1. GET リクエスト（従来の形式）

エンドポイント: GET /get/<user>/<path>

クエリパラメータ:

feeld: 取得したいフィールドを指定します。

rng: データの取得範囲を指定します。

num_of_obj: フィールド内のオブジェクト数を取得する場合に使用します。



2. POST リクエスト（JSON形式による新しい形式）

エンドポイント: POST /get

ヘッダー:

Content-Type: application/json


リクエストボディ: JSON形式で以下のパラメータを含みます。

user: データを取得する対象のユーザー名。

path: 取得したいファイルやデータのパス。

feeld: 取得したいフィールドを指定します。

rng: データの取得範囲を指定します。

num_of_obj: フィールド内のオブジェクト数を取得する場合に使用します。




---

パラメータ詳細

共通パラメータ

user (string, 必須): データを取得する対象のユーザー名。

path (string, 必須): 取得したいファイルやデータのパス。


オプションパラメータ

feeld (string または string の配列): 取得したいフィールドを指定します。ネストされたフィールドはドット記法で指定できます。

例: "feeld": "id"

例: "feeld": ["id", "name", "email"]


rng (string): 取得するデータの範囲を指定します。開始位置-終了位置の形式で指定します。

例: "rng": "3-4"


num_of_obj (boolean): フィールド内のオブジェクト数を取得する場合にtrueを指定します。

例: "num_of_obj": true




---

使用例

1. GET リクエストの例

取得例1: 単一フィールドの取得

リクエスト:

GET /get/john_doe/data.json?feeld=id

レスポンス:

{
  "id": 12345
}


---

取得例2: ネストされたフィールドの取得

リクエスト:

GET /get/john_doe/data.json?feeld=phone_numbers.type

レスポンス:

{
  "phone_numbers": [
    { "type": "home" },
    { "type": "work" }
  ]
}


---

取得例3: 範囲指定での取得

リクエスト:

GET /get/john_doe/data.json?feeld=address&rng=3-4

レスポンス:

{
  "address": {
    "state": "CA",
    "postal_code": "90001"
  }
}


---

2. POST リクエストの例（JSON形式）

取得例1: 単一フィールドの取得

リクエスト:

POST /get
Content-Type: application/json

リクエストボディ:

{
  "user": "john_doe",
  "path": "data.json",
  "feeld": ["id"]
}

レスポンス:

{
  "id": 12345
}


---

取得例2: 複数フィールドの取得

リクエストボディ:

{
  "user": "john_doe",
  "path": "data.json",
  "feeld": ["id", "name", "email"]
}

レスポンス:

{
  "id": 12345,
  "name": "John Doe",
  "email": "johndoe@example.com"
}


---

取得例3: ネストされたフィールドの取得

リクエストボディ:

{
  "user": "john_doe",
  "path": "data.json",
  "feeld": ["preferences.contact_method"]
}

レスポンス:

{
  "preferences": {
    "contact_method": "email"
  }
}


---

取得例4: 範囲指定での取得

リクエストボディ:

{
  "user": "john_doe",
  "path": "data.json",
  "feeld": ["phone_numbers"],
  "rng": "1-1"
}

レスポンス:

{
  "phone_numbers": [
    {
      "type": "home",
      "number": "555-555-1234"
    }
  ]
}


---

取得例5: オブジェクト数の取得

リクエストボディ:

{
  "user": "john_doe",
  "path": "data.json",
  "feeld": ["phone_numbers"],
  "num_of_obj": true
}

レスポンス:

{
  "num_of_obj": 2
}


---

エラー処理とセキュリティ

エラー処理

404 Not Found or Unauthorized:

説明: 対象のデータが存在しない、またはアクセス権限が不足している場合に返されます。

注意: セキュリティ上の理由から、ファイルの存在を秘匿するために403 Forbiddenエラーではなく404 Not Foundエラーを使用しています。


400 Bad Request:

説明: リクエストのパラメータが不正な場合に返されます。

対応: エラーメッセージには具体的な問題点が記載されます。


401 Unauthorized:

説明: 認証に失敗した場合に返されます。

対応: セッションが無効、または認証情報が不足している場合に発生します。



セキュリティ

ファイルの存在を秘匿:

存在しないファイルやアクセス権限がないファイルに対しては、404 Not Foundを返します。これにより、ファイルの存在を第三者に知られないようにします。


パラメータのバリデーション:

すべての入力パラメータはサーバー側で適切にバリデーションされます。特に、userやpathなどのパラメータは、ディレクトリトラバーサル攻撃などを防ぐために検証されます。


セッション管理:

ユーザーの認証と権限チェックはセッション情報に基づいて行われます。


JSON形式でのクエリ送信の推奨:

理由: GETリクエストのクエリパラメータはURLに直接表示されるため、機密性の高い情報を含む場合は不適切です。POSTリクエストでJSON形式のボディを使用することで、情報がURLに露出せず、セキュリティが向上します。

推奨事項: 機密情報や複雑なクエリを送信する場合は、POSTリクエストでJSON形式のクエリを使用することを推奨します。




---

注意事項

feeld パラメータについて:

ネストされたフィールドを指定する場合、ドット記法を使用します。

複数のフィールドを指定する場合は、文字列の配列として渡します。


rng パラメータについて:

データの範囲を指定するために使用します。

インデックスは1から始まります。

例: "rng": "1-3" は最初の3つの要素を取得します。

注意: レンジがデータの要素数を超えている場合でも、取得可能なデータが返されます。


num_of_obj パラメータについて:

trueを指定すると、指定したフィールド内のオブジェクト数を返します。

他のパラメータと組み合わせて使用する場合、レスポンスにはnum_of_objフィールドが含まれます。




---

レスポンス形式

バイナリデータ:

リクエストされたパスがバイナリファイルの場合、Content-Typeヘッダーは適切なバイナリのMIMEタイプになります。


JSONデータ:

Content-Type: application/json

フィールドの指定や範囲の指定に応じて、JSON形式でデータが返されます。




---

まとめ

GET リクエスト:

シンプルなデータ取得に適していますが、URLにパラメータが表示されるため、セキュリティ上の懸念があります。(キャッシュなど)


POST リクエスト（JSON形式）:

複雑なクエリや機密性の高い情報を扱う場合に適しています。

セキュリティの観点からも、情報がURLに露出しないため推奨されます。


