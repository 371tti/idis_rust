# セッション管理について

cookieを使います。

idisのセッション管理は複数ログインを可能にするため、セッションとユーザーセッションを分離して保持します。

## session

src/utils/api/session.rs

で定義されてるの

```rust
pub struct SessionData {
    last_access_time: u64,
    generated_time: u64,
    users: Vec<u128>,
}

pub struct Session {
    sessions: Mutex<HashMap<Vec<u8>, SessionData>>,
    len: usize,
    life_time_server: Duration,
    life_time_client: Duration,
    rng: Mutex<ChaCha20Rng>,
  
}
```

sessionとして `Session.sessions`に全セッションが保管されます

session keyをkeyとしてvalは `SessionData`になります

`SessionData`には以下の情報が含まれます

- 最終アクセス時間
- 生成時間
- ログインしたユーザーリスト

#### 技術

Sessionについて動的な操作を実装する予定はありません。

すべてのSessionをメモリ上に配置します。

メモリ上に配置するかつDBに保存することも、設定で可能にする予定があります。

以上の実装理由はクラスタリング時のセッション共有のためです。

クラスタリング時、セッションデータをtcpで共有するオプションを実装する予定があります。

## user session

sec/utils/api/user.rs

に定義されています

```rust
pub struct UserData {
    user_id: String,
    account_level: i32,
    perm: Vec<u128>,
    latest_access_time: u64, // UTCのミリ秒を格納
}

pub struct User {
    users: Mutex<HashMap<u128, Option<UserData>>>,
    id_to_ruid: Mutex<HashMap<String, u128>>,
    db: MongoClient,
}

```

user sessionとして `User.users`に全セッションが保管されます

user RUIDをkeyとしてvalは   `Option<UserData>`になります

`Option<UserData>`はUserDataまたはNoneが格納されます

`UserData`には以下の情報が含まれます

- ユーザーid ex. @root
- アカウントレベル
- 権限
- 最終アクセス時間

### 技術

UserSession `UserData`は動的にメモリ上から開放するシステムが実装される予定です。

UserSessionはオプションでクラスタリング時、ほかサーバーのupdateイベントにより自動的に更新をする機能が実装される予定です。
