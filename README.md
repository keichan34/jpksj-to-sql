# 国土数値情報データをSQLデータベースに取り込む

このツールは、[国土数値情報](https://nlftp.mlit.go.jp/ksj/)のデータとメタデータをPostgreSQL (PostGIS) 用のデータベースに取り込み、すぐに自由な分析ができる状態に整理します。

取り込むデータは、[JPGIS2.1準拠整備データ一覧](https://nlftp.mlit.go.jp/ksj/gml/gml_datalist.html)から選ばれ、同一データセットに複数ファイルがある場合は1つのテーブルにまとめます（現状は最新年度のみ対応）。

なお、各データセットには「商用」「非商用」「CC BY 4.0」など利用条件が設定されているため、使用時は十分にご注意ください。（今のところ、非商用データはフィルタされています）

## データベースの概要

* データの識別子をテーブル名とし、カラム名は日本語へマッピング後となります。
    * 位置情報は `geom` カラムに入っています
    * Feature ID は `ogc_fid`
* `datasets` テーブルにメタデータが入っています
    * 今の所、拡張性を重視した `table_name TEXT, metadata jsonb` になっています

### メタデータの形

識別子 `P05` からの引用

```jsonc
{
    // データ一覧からの情報
    "data_item": {
        "url": "https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-P05-2022.html",
        "name": "市町村役場等及び公的集会施設",
        // 利用規約
        "usage": "CC_BY_4.0",
        // 識別子
        "identifier": "P05",
        "data_source": "全国、地方、都道府県",
        "metadata_xml": "https://nlftp.mlit.go.jp/ksj/gml/meta/P05/P05-22/KS-META-P05-22_01.xml",
        "data_accuracy": "数値地図25000（地図画像）その他参照した資料と比較。",
        "category1_name": "施設",
        "category2_name": "【施設】"
    },
    // データ詳細ページからの情報
    "data_page": {
        "url": "https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-P05-2022.html",
        // ダウンロード対象のファイル一覧。この場合は全国データがあったので、全国の1ファイルのみ読み込んでいます。
        "items": [
            {
                "crs": "世界測地系",
                "area": "全国",
                "year": "2022年（令和4年）",
                // ファイル一覧のバイト数から計算している。おおよそ
                "bytes": 7980000,
                "nendo": null,
                "file_url": "https://nlftp.mlit.go.jp/ksj/gml/data/P05/P05-22/P05-22_GML.zip"
            }
        ],
        "metadata": {
            "attribute": {
                // shapefile内の値名
                "P05_001": {
                    // データベース上のカラム名
                    "name": "行政区域コード",
                    // 参照先情報がある場合はそのリンク。ない場合は「null」
                    // TODO: このデータも取り込む
                    "ref_url": "https://nlftp.mlit.go.jp/ksj/gml/codelist/AdminiBoundary_CD.xlsx",
                    // 値の型
                    "attr_type": "コードリスト「行政区域コード」",
                    "description": "都道府県コードと市区町村コードからなる、行政区を特定するためのコード"
                },
                // ...
            },
            // ページ上部にある表の内容。表をそのまま読み込んでいるので共通の値はあるが正規化はしていない。
            "fundamental": {
                "内容": "全国の市役所、区役所、町役場、村役場、及びこれらの支所、出張所、連絡所等、及び市区町村が主体的に設置・管理・運営する公民館、集会所等の公的集会施設について、その位置と名称、所在地、施設分類コード、行政コードをGISデータとして整備したものである。",
                "座標系": "JGD2011/（B,L）",
                "原典資料": "・2010年度（平成22年度）国土数値情報（市町村役場等及び公的集会施設）作成業務 業務報告書（冊子）\n・（社）全国公民館連合会「2005年（平成17年）版 全国公民館名鑑」\n・その他、業務に必要となる資料、作業手順書及び製品仕様書等一式",
                "更新履歴": "2023年：2022年（令和4年）版更新\n2011年：2010年（平成22年）版更新",
                "データ形状": "点",
                "関連する法律": "－",
                "データの基準年月日": "「2022年（令和4年）4月」",
                "作成方法（原典表示）": "各市区町村の開設する公式ウェブサイト及び全国公民館名鑑から、当該施設の情報を取得し、市町村役場については国土数値情報（公共施設）から市役所のデータを取得し、他の施設については所在地情報から、街区レベル位置参照情報を用いてジオコーディングを行い、数値地図25000（地図画像）から、その正確な位置を取得し整備した。\n「この地図の作成にあたっては、国土地理院長の承認を得て、同院発行の数値地図25000（地図画像）を使用した。",
                "このデータの使用許諾条件": "適用する利用規約に基づく（オープンデータ）"
            }
        }
    }
}
```

## 利用方法

バイナリを [最新リリース](https://github.com/keichan34/jpksj-to-sql/releases/) からダウンロードするのがおすすめです。

GDAL 3.9以上必要です (`ogr2ogr` または `ogrinfo` が実行できる環境。 `ogrinfo` は `-limit` 引数使うので、 3.9 が必要です)

```
jpksj-to-sql "host=127.0.0.1 dbname=jpksj"
```

macOS の場合: Gatekeeper の設定で GitHub Release でダウンロードしたバイナリを実行できない場合があります。 `xattr -d com.apple.quarantine ./jpksj-to-sql` を実行したら突破できます。

インターネット接続、メモリ、SSD転送速度等によって処理時間が大幅に左右します。途中からの続きを再開するために幾つかのオプションがあるので、 `jpksj-to-sql --help` で確認してください。

ダウンロードした ZIP ファイルや解凍した shapefile をデフォルトで実行ディレクトリ内 `./tmp` に保存されます。

## コンパイル

Rust の開発環境が必要です。構築後、 cargo を使ってください

```
cargo build
```

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
- [ ] 同一データセット内に複数識別子が存在する時のハンドリング（PR歓迎）
- [ ] 複数年のデータが存在する場合のハンドリング（PR歓迎）
- [ ] PostgreSQL以外のデータベースにも保存（PR歓迎）
- [ ] 部分更新（必要ないかも）

## ライセンス

こちらのレポジトリのソースコードには MIT ライセンスが適用します。
