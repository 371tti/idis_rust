# IDIS_rust

~~python x Flask x mongoDB
の楽しいマイクロブロク系SNS~~

はパフォーマンスの都合でプロジェクト崩壊したので、rustでより効率的に動作するよう、作り直しているもの。

マイクロブロクSNSを目指して現在開発中。

## idis 概要

* ファイルシステム風
* シンプルでわかりやすい
* サードパーティーによる拡張がしやすい
* 高機能な権限管理
* 高性能なデータ管理

以上が目標


idisの根本的な部分はストレージであり、完結かつ効率化された非常に汎用的なAPIを提供することにより、SNSとして稼働させるもの

さらにユーザーはidis内でapiを仕様するシステムを構築することができ、サードパーティによる機能の拡張を促進する
