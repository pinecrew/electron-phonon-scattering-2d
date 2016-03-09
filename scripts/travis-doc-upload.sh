#!/bin/sh

# License: CC0 1.0 Universal
# https://creativecommons.org/publicdomain/zero/1.0/legalcode

set -e

. scripts/travis-doc-upload.cfg

[ "$TRAVIS_BRANCH" = master ]

[ "$TRAVIS_PULL_REQUEST" = false ]

eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/id_rsa.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

git clone --branch gh-pages git@github.com:$TRAVIS_REPO_SLUG deploy_docs

cd deploy_docs
git config user.name "doc upload bot"
git config user.email "nobody@example.com"
rm -rf *

docs=$(find ../target/doc/ -name "*.html")
for file in $docs; do
    echo $file
    sed -i 's_\(</head>\)_<link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/KaTeX/0.5.1/katex.min.css">\n<script src="//cdnjs.cloudflare.com/ajax/libs/KaTeX/0.5.1/katex.min.js"></script>\n<script src="//cdnjs.cloudflare.com/ajax/libs/KaTeX/0.5.1/contrib/auto-render.min.js"></script>\n\1_' $file
    sed -i 's_\(</body>\)_<script>renderMathInElement(document.getElementById("math"),{delimiters: [{left: "$$", right: "$$", display: true},{left: "$", right: "$", display: false}]});</script>_' $file
    sed -i 's_<body_<body id=math_' $file
done

mv ../target/doc/* .
git add -A .
git commit -qm "doc upload"
git push -q origin gh-pages
