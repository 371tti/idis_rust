# jsonによるqueryフォーマット

test json
```json
{
  "string": "Hello, world!",         // 文字列型
  "number": 12345,                  // 数値型（整数）
  "float": 123.45,                  // 数値型（浮動小数点数）
  "boolean_true": true,             // 真偽値型（true）
  "boolean_false": false,           // 真偽値型（false）
  "null_value": null,               // null型
  "array": [                        // 配列型
    "string in array",
    42,
    true,
    null,
    { "nested_object_in_array": "value" }
  ],
  "nested_object": {                // ネストされたオブジェクト型
    "nested_string": "Nested Hello!",
    "nested_number": 6789,
    "nested_array": [1, 2, 3],
    "nested_null": null
  },
  "empty_object": {},               // 空のオブジェクト型
  "empty_array": [],                // 空の配列型
  "special_characters": "@#$%^&*()_+", // 特殊文字を含む文字列
  "unicode_string": "こんにちは世界", // Unicode文字列
  "long_number": 123456789012345678901234567890, // 長い数値（丸めに注意）
  "scientific_notation": 1.23e+10,  // 科学的記数法表現
  "boolean_array": [true, false, true], // 真偽値を含む配列
  "nested_objects_in_array": [      // 配列の中にオブジェクト
    { "key1": "value1" },
    { "key2": "value2" }
  ]
}

```

query json
```json
{
    "nested_object": {
         "nested_number": true,
         "nested_array": ["$0..1"],
    }
}
```
result json
```json
{
    "nested_object": {                // ネストされたオブジェクト型
        "nested_number": 6789,
        "nested_array": [1, 2]
  }
}
```
型非限定
- ドット記法: "key.key.**"
- ドット記法: "key.index"
- object type: "key": bool
<!-- - object type: "$": "$qery formula" -->
- list type: "index"
- list type: "$qery formula"

ダブった場合はorで処理 おなじ要素でfalseがより後ろで指定された場合は除外する