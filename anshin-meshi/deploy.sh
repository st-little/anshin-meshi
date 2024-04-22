#!/bin/sh

echo "Start deploy." `date '+%y/%m/%d %H:%M:%S'`

# .env ファイルから APP_VERSION を取得する関数
get_app_version() {
    # .env ファイルが存在するかどうかを確認
    if [ -f .env ]; then
        # .env ファイルから APP_VERSION の値を grep で取得し、sed で整形して表示
        version=$(grep "^APP_VERSION=" .env | sed 's/APP_VERSION=//')
        if [ -z "$version" ]; then
            echo "エラー: .env ファイルに APP_VERSION が見つかりません。"
            exit 1
        fi
        echo "$version"
    else
        echo "エラー: .env ファイルが見つかりません。"
        exit 1
    fi
}

# APP_VERSION を取得して表示
app_version=$(get_app_version)
if [ $? -eq 0 ]; then
    echo "APP_VERSION: $app_version"
else
    echo "Failed get APP_VERSION." `date '+%y/%m/%d %H:%M:%S'`
    exit 1
fi

# Build
echo "Start build." `date '+%y/%m/%d %H:%M:%S'`
dx build --release
if [ $? -eq 0 ]; then
    echo "Completed build." `date '+%y/%m/%d %H:%M:%S'`
else
    echo "Failed build." `date '+%y/%m/%d %H:%M:%S'`
    exit 1
fi

# Push to gh-pages branch
echo "Start push." `date '+%y/%m/%d %H:%M:%S'`
git subtree push --prefix dist/ origin gh-pages
git tag "v$app_version" gh-pages
echo "Completed push." `date '+%y/%m/%d %H:%M:%S'`

echo "Completed deploy." `date '+%y/%m/%d %H:%M:%S'`