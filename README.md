# 国土数値情報をSQLデータベースに読み込む

このツールは[国土数値情報](https://nlftp.mlit.go.jp/ksj/)のデータをすべて読み込み、データベースに読み込みます。
現在、PostgreSQL (PostGIS) のデータベースの想定で作っています。

読み込むデータ一覧は、 [JPGIS2.1準拠整備データ一覧](https://nlftp.mlit.go.jp/ksj/gml/gml_datalist.html) から読み込まれます。同一データセットに複数ファイルがある場合（ほとんど）、1テーブルにまとめます。今のところ、最新年度のデータしか読み込まれていません。

データセットによって利用用途は「商用」「非商用」「CC BY 4.0」などの制限があるので、データを利用するにあたって十分気をつけてください。データベースにメタデータのテーブルにもあるので、利用ください。「非商用」は現在、フィルターされています。取り組みたい方はPR歓迎です。

## 利用方法

GDAL 2.1以上必要です (`ogr2ogr` または `ogrinfo` が実行できる環境)

```
jpksg-to-sql "host=127.0.0.1 dbname=jpksg"
```

インターネット接続、メモリ、SSD転送速度等によって処理時間が大幅に左右します。途中からの続きを再開するために幾つかのオプションがあるので、 `jpksg-to-sql --help` で確認してください。

ダウンロードした ZIP ファイルや解凍した shapefile をデフォルトで実行ディレクトリ内 `./tmp` に保存されます。

## ステータス

こちらは実験的なツールであり、商用運用はお勧めしておりません。データの実験のために使われているので適当な実装が多いのですが、機能について下記をご覧ください。
「PR歓迎」となっているところは挑戦してみてください。

- [x] 国土数値情報のウェブサイトからデータ一覧の取得
- [x] データのダウンロードおよび解凍
- [x] メタデータの取得およびパース（ `shape_property_table2.xlsx` ）
- [x] メタデータを元に属性名マッピング（shapefileでの `G04a_001` みたいなのを SQL カラム名 `3次メッシュコード` に変換）
- [x] メタデータをデータベース内に保存（ `datasets` テーブル参照してください ）
- [x] 読み込むデータセットの指定
- [x] 文字コードの認識
- [ ] VRTによるレイヤー統合から並列処理に変更
- [ ] 同一データセット内に複数識別子が存在する時のハンドリング（そのうちやります、、）
- [ ] 複数年のデータが存在する場合のハンドリング（PR歓迎）
- [ ] PostgreSQL以外のデータベースにも保存（PR歓迎）
- [ ] 部分更新（必要ないかも）

## ライセンス

こちらのレポジトリのソースコードには MIT ライセンスが適用します。
