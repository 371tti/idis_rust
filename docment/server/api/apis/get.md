# `get/<user>/<path>` -> BinaryStream

これは、ユーザー名とpathを指定し、データを返却します。
内部ではセッションから権限を取得し、処理を行っています。

一般的なエラーとして

- 404 - NotFound_or_Unauthorized
  
があります。
403は存在しません。これはファイルの存在を秘匿するためです。

## option of get

json_fileをgetする場合、クエリでフィールドやページを指定できます

`get/<user>/<json_path>?feeld=<feeld_qery>&rng=<objects_reange>&num_of_obj=<bool>`
ex.

```json
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

```

このjsonを取得する場合
`?feeld=id` ->

```json
{"id": 12345}
```

`?feeld=phone_numbers.type` ->

```json
{
    "phone_numbers": [
        {"type": "home"}
    ]
}
```

`?feeld=address&rng=3-4` ->

```json
{
  "address": {
    "state": "CA",
    "postal_code": "90001"
  }
}
```

`?feeld=address&num_of_obj=true` ->

```json
{
  "num_of_obj": 4
}
```

レンジが要素数を超えた場合、一般的な返却と変化はありません。
